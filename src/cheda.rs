///! Defines Python bindings for `vidyut_cheda`.
use vidyut_cheda::{Chedaka, Config, Error};

use crate::kosha::semantics::PyPada;
use pyo3::exceptions::{PyOSError, PyValueError};
use pyo3::prelude::*;
use std::path::PathBuf;

/// A token.
#[pyclass(name = "Token", get_all)]
pub struct PyToken {
    /// The token text.
    pub text: String,
    /// Other information associated with the token.
    pub info: PyPada,
}

#[pymethods]
impl PyToken {
    #[getter]
    fn lemma(&self) -> Option<String> {
        self.info.lemma()
    }

    fn __repr__(&self) -> String {
        format!(
            "Token<(text=\'{}\', lemma='{}', info={})>",
            self.text,
            self.lemma().unwrap_or_default(),
            self.info.__repr__()
        )
    }
}

/// A Sanskrit segmentation engine.
#[pyclass(name = "Chedaka")]
pub struct PyChedaka {
    chedaka: Chedaka,
}

#[pymethods]
impl PyChedaka {
    /// Initialize `Chedaka` by reading the necessary data from the directory at `path`.
    ///
    /// This constructor raises a ValueError if the initialiation fails.
    #[new]
    fn new(path: PathBuf) -> PyResult<Self> {
        let config = Config::new(path);
        match Chedaka::new(config) {
            Ok(chedaka) => Ok(PyChedaka { chedaka }),
            Err(e) => Err(WrappedError(e).into()),
        }
    }

    /// Parse the given SLP1 input and return a list of `Token` objects.
    pub fn run(&self, slp1_text: &str) -> PyResult<Vec<PyToken>> {
        let tokens = match self.chedaka.run(slp1_text) {
            Ok(tokens) => tokens,
            Err(e) => return Err(WrappedError(e).into()),
        };

        let mut ret = Vec::new();
        for token in tokens {
            ret.push(PyToken {
                text: token.text().to_string(),
                info: token.info().clone().into(),
            });
        }

        Ok(ret)
    }
}

struct WrappedError(Error);

impl From<Error> for WrappedError {
    fn from(e: Error) -> Self {
        Self(e)
    }
}

impl From<WrappedError> for PyErr {
    fn from(e: WrappedError) -> Self {
        match e.0 {
            Error::Io(e) => PyOSError::new_err(format!("{}", e)),
            e => PyValueError::new_err(format!("{}", e)),
        }
    }
}
