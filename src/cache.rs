// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use pyo3::prelude::*;
use pyo3::types::PyString;
use std::collections::HashMap;

pub struct KeyCache {
    cache: HashMap<String, Py<PyString>>,
}

impl KeyCache {
    pub fn new() -> Self {
        KeyCache {
            cache: HashMap::new(),
        }
    }

    pub fn get_or_create<'py>(&mut self, py: Python<'py>, key: &str) -> Py<PyString> {
        if let Some(py_obj) = self.cache.get(key) {
            return py_obj.clone_ref(py);
        }

        let py_str: Py<PyString> = PyString::new(py, key).into();
        self.cache.insert(key.to_string(), py_str.clone_ref(py));
        py_str
    }
}