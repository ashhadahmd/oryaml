// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::cache::KeyCache;
use crate::error::Error;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyList, PyString, PyTuple};
use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, Visitor};
use std::fmt;

const MAX_RECURSION_DEPTH: usize = 254;

// Shared state for one loads() call. Errors that aren't YAML syntax errors
// (Python exceptions, recursion, unhashable keys) are stashed here so they
// surface with their real exception type rather than as a serde message.
struct Ctx<'py> {
    py: Python<'py>,
    cache: KeyCache,
    pending: Option<Error>,
}

impl Ctx<'_> {
    fn fail<E: de::Error>(&mut self, err: Error) -> E {
        self.pending = Some(err);
        E::custom("aborted")
    }
}

pub fn deserialize(py: Python, data: &str) -> Result<Py<PyAny>, Error> {
    let mut ctx = Ctx { py, cache: KeyCache::new(), pending: None };
    let seed = ValueSeed { ctx: &mut ctx, depth: 0, key: false };
    match seed.deserialize(serde_yaml::Deserializer::from_str(data)) {
        Ok(obj) => Ok(obj),
        Err(e) => Err(ctx.pending.take().unwrap_or(Error::Yaml(e))),
    }
}

// Builds Python objects straight from parser events, without an intermediate
// serde_yaml::Value tree. `key` marks a mapping key, which must be hashable:
// sequences become tuples and mappings are rejected.
struct ValueSeed<'a, 'py> {
    ctx: &'a mut Ctx<'py>,
    depth: usize,
    key: bool,
}

impl<'de> DeserializeSeed<'de> for ValueSeed<'_, '_> {
    type Value = Py<PyAny>;

    fn deserialize<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'py> ValueSeed<'_, 'py> {
    fn child(&mut self, key: bool) -> ValueSeed<'_, 'py> {
        ValueSeed { ctx: &mut *self.ctx, depth: self.depth + 1, key }
    }

    fn check_depth<E: de::Error>(&mut self) -> Result<(), E> {
        if self.depth >= MAX_RECURSION_DEPTH {
            return Err(self.ctx.fail(Error::Recursion));
        }
        Ok(())
    }
}

impl<'de> Visitor<'de> for ValueSeed<'_, '_> {
    type Value = Py<PyAny>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("any YAML value")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(self.ctx.py.None())
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(self.ctx.py.None())
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
        Ok(PyBool::new(self.ctx.py, v).to_owned().into_any().unbind())
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
        Ok(v.into_pyobject(self.ctx.py).unwrap().into_any().unbind())
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
        Ok(v.into_pyobject(self.ctx.py).unwrap().into_any().unbind())
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E> {
        Ok(v.into_pyobject(self.ctx.py).unwrap().into_any().unbind())
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E> {
        Ok(v.into_pyobject(self.ctx.py).unwrap().into_any().unbind())
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
        Ok(PyFloat::new(self.ctx.py, v).into_any().unbind())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
        if self.key {
            Ok(self.ctx.cache.get_or_create(self.ctx.py, v).into_any())
        } else {
            Ok(PyString::new(self.ctx.py, v).into_any().unbind())
        }
    }

    fn visit_seq<A: SeqAccess<'de>>(mut self, mut seq: A) -> Result<Self::Value, A::Error> {
        self.check_depth()?;
        let key = self.key;
        let mut items = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(item) = seq.next_element_seed(self.child(key))? {
            items.push(item);
        }
        let py = self.ctx.py;
        let obj = if key {
            PyTuple::new(py, items).map(|t| t.into_any())
        } else {
            PyList::new(py, items).map(|l| l.into_any())
        };
        obj.map(Bound::unbind).map_err(|e| self.ctx.fail(e.into()))
    }

    fn visit_map<A: MapAccess<'de>>(mut self, mut map: A) -> Result<Self::Value, A::Error> {
        if self.key {
            return Err(self.ctx.fail(Error::Type("Mapping keys are not supported: a dict is not hashable".to_string())));
        }
        self.check_depth()?;
        let dict = PyDict::new(self.ctx.py);
        while let Some(k) = map.next_key_seed(self.child(true))? {
            let v = map.next_value_seed(self.child(false))?;
            let len = dict.len();
            if let Err(e) = dict.set_item(k, v) {
                return Err(self.ctx.fail(e.into()));
            }
            if dict.len() == len {
                return Err(de::Error::custom("duplicate mapping key"));
            }
        }
        Ok(dict.into_any().unbind())
    }

    // serde_yaml reports local tags (`!Foo`) as enums. Standard `!!` tags are
    // resolved or stripped by serde_yaml before reaching this visitor.
    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        let (tag, _) = data.variant::<String>()?;
        Err(de::Error::custom(format!("unsupported YAML tag '!{tag}'")))
    }
}
