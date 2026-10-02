// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};

mod cache;
mod deserialize;
mod error;
mod serialize;

use crate::deserialize::deserialize;
use crate::serialize::{serialize, serialize_stream, serialize_to_path};

// loads(doc, /)
// Deserialize YAML from str or bytes to a Python object.
#[pyfunction]
fn loads(py: Python, input_data: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    if let Ok(bytes) = input_data.cast::<PyBytes>() {
        let slice = bytes.as_bytes();
        let s = match std::str::from_utf8(slice) {
            Ok(s) => s,
            Err(e) => return Err(pyo3::exceptions::PyUnicodeDecodeError::new_utf8(py, slice, e)?.into()),
        };

        return deserialize(py, s).map_err(PyErr::from);
    }

    if let Ok(s) = input_data.cast::<PyString>() {
        let s_slice = s.to_str()?;
        return deserialize(py, s_slice).map_err(PyErr::from);
    }

    Err(pyo3::exceptions::PyTypeError::new_err("Argument 'doc' must be str or bytes"))
}

// dumps(obj, /, stream=None)
// Serialize a Python object to YAML bytes, or write it to `stream`.
#[pyfunction]
#[pyo3(signature = (data, stream=None))]
fn dumps(py: Python, data: Bound<'_, PyAny>, stream: Option<Bound<'_, PyAny>>) -> PyResult<Option<Py<PyBytes>>> {
    match stream {
        Some(s) => {
            if let Ok(path_str) = s.cast::<PyString>() {
                let path = path_str.to_str()?;
                serialize_to_path(py, &data, path).map_err(PyErr::from)?;
                return Ok(None);
            }

            if s.hasattr("write")? {
                serialize_stream(py, &data, &s).map_err(PyErr::from)?;
                return Ok(None);
            }

            Err(pyo3::exceptions::PyTypeError::new_err("Argument 'stream' must be a file path (str) or a file-like object"))
        },
        None => {
            let bytes = serialize(py, &data).map_err(PyErr::from)?;
            Ok(Some(PyBytes::new(py, &bytes).into()))
        }
    }
}

#[pymodule]
fn oryaml(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(loads, m)?)?;
    m.add_function(wrap_pyfunction!(dumps, m)?)?;
    Ok(())
}