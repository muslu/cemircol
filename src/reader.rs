// src/reader.rs
use flate2::read::ZlibDecoder;
use memmap2::Mmap;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::fs::File;
use std::io::Read;

use crate::writer::FileMeta;

#[pyclass]
pub struct CemircolReader {
    mmap: Mmap,
    metadata: FileMeta,
}

#[pymethods]
impl CemircolReader {
    #[new]
    fn new(file_path: &str) -> PyResult<Self> {
        let file = File::open(file_path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

        let mmap = unsafe {
            Mmap::map(&file)
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?
        };

        let len = mmap.len();
        if len < 16 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Invalid file: too small",
            ));
        }

        // Validate closing magic
        if &mmap[len - 4..] != b"CEM1" {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Invalid file format: missing CEM1 magic",
            ));
        }

        // Read metadata length (u64 LE)
        let meta_len_bytes: [u8; 8] = mmap[len - 12..len - 4]
            .try_into()
            .map_err(|_| {
                pyo3::exceptions::PyValueError::new_err("Failed to read metadata length")
            })?;
        let meta_len = u64::from_le_bytes(meta_len_bytes) as usize;

        // Read and parse metadata JSON
        let meta_start = len - 12 - meta_len;
        let meta_bytes = &mmap[meta_start..meta_start + meta_len];
        let metadata: FileMeta = serde_json::from_slice(meta_bytes).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Invalid metadata: {}", e))
        })?;

        Ok(Self { mmap, metadata })
    }

    /// Query a single column by name. Returns a list of int or float values.
    fn query<'py>(&self, py: Python<'py>, column: &str) -> PyResult<Bound<'py, PyList>> {
        let col_meta =
            self.metadata.columns.get(column).ok_or_else(|| {
                pyo3::exceptions::PyKeyError::new_err(format!("Column '{}' not found", column))
            })?;

        let start = col_meta.offset as usize;
        let end = start + col_meta.compressed_length as usize;
        let compressed = &self.mmap[start..end];

        // Decompress
        let mut decoder = ZlibDecoder::new(compressed);
        let mut decompressed = Vec::with_capacity(col_meta.uncompressed_length as usize);
        decoder.read_to_end(&mut decompressed).map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("Decompression error: {}", e))
        })?;

        // Convert bytes to Python list based on data type
        match col_meta.data_type.as_str() {
            "int64" => {
                let values: Vec<i64> = decompressed
                    .chunks_exact(8)
                    .map(|chunk| i64::from_le_bytes(chunk.try_into().unwrap()))
                    .collect();
                PyList::new(py, &values)
            }
            "float64" => {
                let values: Vec<f64> = decompressed
                    .chunks_exact(8)
                    .map(|chunk| f64::from_le_bytes(chunk.try_into().unwrap()))
                    .collect();
                PyList::new(py, &values)
            }
            _ => Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "Unsupported data type: {}",
                col_meta.data_type
            ))),
        }
    }

    /// Return list of column names.
    fn columns(&self) -> Vec<String> {
        self.metadata.columns.keys().cloned().collect()
    }

    /// Return the number of rows.
    fn num_rows(&self) -> u64 {
        self.metadata.num_rows
    }
}
