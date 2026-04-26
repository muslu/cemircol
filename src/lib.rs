// src/lib.rs
mod reader;
mod writer;

use pyo3::prelude::*;

/// CemirCol — High-performance columnar storage with zlib compression.
#[pymodule]
fn _cemircol(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<writer::CemircolWriter>()?;
    m.add_class::<reader::CemircolReader>()?;
    Ok(())
}