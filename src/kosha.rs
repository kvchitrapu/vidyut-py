use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use semantics::PyPada;
use std::path::PathBuf;
use vidyut_kosha as rust;

pub mod semantics;

/// A compact Sanskrit kosha.
#[pyclass]
pub struct Kosha(rust::Kosha);

#[pymethods]
impl Kosha {
    /// Load a `Kosha` instance from the given input path.
    #[new]
    fn new(path: PathBuf) -> PyResult<Self> {
        match rust::Kosha::new(&path) {
            Ok(k) => Ok(Self(k)),
            Err(_) => Err(PyOSError::new_err(
                "Unknown error. Our best guess is that the input file is missing.",
            )),
        }
    }

    /// Return whether the kosha contains `key`.
    pub fn __contains__(&self, key: String) -> bool {
        self.0.contains_key(&key)
    }

    /// Return whether the kosha contains at least one key with prefix `prefix`.
    pub fn contains_prefix(&self, prefix: String) -> bool {
        self.0.contains_prefix(&prefix)
    }

    /// Return all entries with the given `key`.
    pub fn get_all(&self, key: String) -> Vec<PyPada> {
        let results = self.0.get_all(&key);
        results
            .iter()
            .flat_map(|p| {
                if let Ok(pada) = self.0.unpack(p) {
                    Some(pada.into())
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Builder for a `Kosha`.
///
/// Memory usage is linear in the number of unique lemmas.
#[pyclass]
pub struct Builder {
    builder: Option<rust::Builder>,
}

#[pymethods]
impl Builder {
    /// Create a new builder whose output will be written to `path`.
    ///
    /// If `path` does not exist, the builder will create it.
    #[new]
    fn new(path: PathBuf) -> PyResult<Self> {
        match rust::Builder::new(&path) {
            Ok(k) => Ok(Self { builder: Some(k) }),
            Err(_) => Err(PyOSError::new_err(
                "Unknown error. Our guess is that the input file is missing.",
            )),
        }
    }

    /// Insert the given (`key`, `pada`) pair.
    ///
    /// Keys must be inserted in lexicographic order. If a key is received out of order,
    /// this method will raise an `OSError`.
    fn insert(&mut self, key: String, pada: PyPada) -> PyResult<()> {
        match self.builder {
            Some(ref mut b) => match b.insert(&key, &pada.into()) {
                Ok(()) => Ok(()),
                Err(_) => Err(PyOSError::new_err("Could not write key.")),
            },
            None => Err(PyOSError::new_err("Kosha has already been written.")),
        }
    }

    /// Complete the build process.
    ///
    /// If this method is not called, the output data will be invalid.
    fn finish(&mut self) -> PyResult<()> {
        if let Some(x) = self.builder.take() {
            let builder = x;

            match builder.finish() {
                Ok(()) => Ok(()),
                Err(_) => Err(PyOSError::new_err("Could not write kosha.")),
            }
        } else {
            Err(PyOSError::new_err("Kosha has already been written."))
        }
    }
}
