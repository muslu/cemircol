// src/writer.rs
use flate2::write::ZlibEncoder;
use flate2::Compression;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Seek, Write};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ColumnMeta {
    pub offset: u64,
    pub compressed_length: u64,
    pub uncompressed_length: u64,
    pub data_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileMeta {
    pub num_rows: u64,
    pub columns: HashMap<String, ColumnMeta>,
}

#[pyclass]
pub struct CemircolWriter;

#[pymethods]
impl CemircolWriter {
    #[staticmethod]
    fn write(filename: &str, data: &Bound<'_, PyDict>) -> PyResult<()> {
        if data.is_empty() {
            return Ok(());
        }

        let mut file = File::create(filename)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

        // Write opening magic bytes
        file.write_all(b"CEM1")
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

        let mut meta = FileMeta {
            num_rows: 0,
            columns: HashMap::new(),
        };

        let mut first = true;

        for (key, value) in data.iter() {
            let col_name: String = key.extract()?;

            // Try int64 first, then float64
            let (raw_bytes, dtype) = if let Ok(values) = value.extract::<Vec<i64>>() {
                if first {
                    meta.num_rows = values.len() as u64;
                    first = false;
                } else if values.len() as u64 != meta.num_rows {
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "Column '{}' length mismatch",
                        col_name
                    )));
                }
                let bytes: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
                (bytes, "int64")
            } else if let Ok(values) = value.extract::<Vec<f64>>() {
                if first {
                    meta.num_rows = values.len() as u64;
                    first = false;
                } else if values.len() as u64 != meta.num_rows {
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "Column '{}' length mismatch",
                        col_name
                    )));
                }
                let bytes: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
                (bytes, "float64")
            } else {
                return Err(pyo3::exceptions::PyTypeError::new_err(format!(
                    "Column '{}': unsupported type (expected list of int or float)",
                    col_name
                )));
            };

            // Compress with zlib level 9
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
            encoder
                .write_all(&raw_bytes)
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
            let compressed = encoder
                .finish()
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

            let offset = file
                .stream_position()
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

            file.write_all(&compressed)
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

            meta.columns.insert(
                col_name,
                ColumnMeta {
                    offset,
                    compressed_length: compressed.len() as u64,
                    uncompressed_length: raw_bytes.len() as u64,
                    data_type: dtype.to_string(),
                },
            );
        }

        // Write metadata footer
        let meta_json = serde_json::to_vec(&meta)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        file.write_all(&meta_json)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

        // Write metadata length (u64 LE)
        file.write_all(&(meta_json.len() as u64).to_le_bytes())
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

        // Write closing magic bytes
        file.write_all(b"CEM1")
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

        Ok(())
    }
}
