///! Defines Python bindings for `vidyut_cheda`.
use vidyut_cheda::{Chedaka, Config};

use crate::kosha::semantics::PyPada;
use pyo3::exceptions::PyValueError;
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
            self.lemma().unwrap_or("".to_string()),
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
        let chedaka = Chedaka::new(config);
        match chedaka {
            Ok(chedaka) => Ok(PyChedaka { chedaka }),
            Err(e) => Err(PyValueError::new_err(format!(
                "Could not initialize segmenter. Error was: `{:?}`",
                e
            ))),
        }
    }

    /// Parse the given SLP1 input and return a list of `Token` objects.
    pub fn run(&self, slp1_text: &str) -> Vec<PyToken> {
        let tokens = self.chedaka.run(slp1_text);
        let mut ret = Vec::new();

        for token in tokens {
            ret.push(PyToken {
                text: token.text.clone(),
                info: token.info.into(),
            });
        }

        ret
    }
}
