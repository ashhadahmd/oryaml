// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use pyo3::prelude::*;
use pyo3::exceptions::{PyRecursionError, PyTypeError, PyValueError, PyOverflowError};
use std::io;

#[derive(Debug)]
pub enum Error {
    Yaml(serde_yaml::Error),
    Py(PyErr),
    Type(String),
    Overflow(String),
    Recursion,
    Io(io::Error),
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::Yaml(err)
    }
}

impl From<PyErr> for Error {
    fn from(err: PyErr) -> Error {
        Error::Py(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        match err {
            Error::Yaml(e) => PyValueError::new_err(format!("YAML Parse Error: {}", e)),
            Error::Py(e) => e,
            Error::Type(msg) => PyTypeError::new_err(msg),
            Error::Overflow(msg) => PyOverflowError::new_err(msg),
            Error::Recursion => PyRecursionError::new_err("Recursion limit reached"),
            Error::Io(e) => e.into(),
        }
    }
}