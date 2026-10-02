// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::error::Error;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyBytes, PyDict, PyFloat, PyInt, PyList, PyString, PyTuple};
use serde_yaml::Value;
use std::fs::File;
use std::io::Write;

const MAX_RECURSION_DEPTH: usize = 254;

pub fn serialize(py: Python, obj: &Bound<'_, PyAny>) -> Result<Vec<u8>, Error> {
    let yaml_value = to_rust_value(py, obj, 0)?;
    let mut buffer = Vec::new();
    serde_yaml::to_writer(&mut buffer, &yaml_value)?;
    Ok(buffer)
}

pub fn serialize_to_path(py: Python, obj: &Bound<'_, PyAny>, path: &str) -> Result<(), Error> {
    let buffer = serialize(py, obj)?;
    let mut file = File::create(path)?;
    file.write_all(&buffer)?;

    Ok(())
}

pub fn serialize_stream(py: Python, obj: &Bound<'_, PyAny>, stream: &Bound<'_, PyAny>) -> Result<(), Error> {
    // Serialize fully before touching the stream so that Python exceptions
    // raised by write()/flush() propagate unchanged instead of being
    // stringified through io::Error.
    let buffer = serialize(py, obj)?;
    stream.call_method1("write", (PyBytes::new(py, &buffer),))?;
    if stream.hasattr("flush")? {
        stream.call_method0("flush")?;
    }
    Ok(())
}

fn to_rust_value(py: Python, obj: &Bound<'_, PyAny>, depth: usize) -> Result<Value, Error> {
    if depth > MAX_RECURSION_DEPTH {
        return Err(Error::Recursion);
    }

    if obj.is_none() {
        return Ok(Value::Null);
    }

    if let Ok(b) = obj.cast::<PyBool>() {
        return Ok(Value::Bool(b.is_true()));
    }

    if let Ok(s) = obj.cast::<PyString>() {
        return Ok(Value::String(s.to_string()));
    }

    if let Ok(l) = obj.cast::<PyList>() {
        let mut seq = Vec::with_capacity(l.len());
        for item in l {
            seq.push(to_rust_value(py, &item, depth + 1)?);
        }
        return Ok(Value::Sequence(seq));
    }

    if let Ok(t) = obj.cast::<PyTuple>() {
        let mut seq = Vec::with_capacity(t.len());
        for item in t {
            seq.push(to_rust_value(py, &item, depth + 1)?);
        }
        return Ok(Value::Sequence(seq));
    }

    if let Ok(d) = obj.cast::<PyDict>() {
        let mut map = serde_yaml::Mapping::new();
        for (k, v) in d {
            let key = to_rust_value(py, &k, depth + 1)?;
            let val = to_rust_value(py, &v, depth + 1)?;
            map.insert(key, val);
        }
        return Ok(Value::Mapping(map));
    }

    if let Ok(f) = obj.cast::<PyFloat>() {
        return Ok(Value::Number(serde_yaml::Number::from(f.value())));
    }

    if let Ok(i) = obj.extract::<i64>() {
        return Ok(Value::Number(serde_yaml::Number::from(i)));
    }

    if let Ok(u) = obj.extract::<u64>() {
        return Ok(Value::Number(serde_yaml::Number::from(u)));
    }

    if obj.is_instance_of::<PyInt>() {
        return Err(Error::Overflow(format!(
            "Integer exceeds 64-bit range supported by oryaml: {}",
            obj.repr()?
        )));
    }

    Err(Error::Type(format!("Object of type '{}' is not serializable", obj.get_type().name()?)))
}
