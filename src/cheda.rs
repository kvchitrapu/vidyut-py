use vidyut_cheda::{Chedaka, Config};

use crate::common::PyPada;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::path::PathBuf;

/// A parsed word.
#[pyclass(name = "Token")]
pub struct PyToken {
    /// The token text.
    #[pyo3(get)]
    pub text: String,
    /// The token lemma.
    #[pyo3(get)]
    pub lemma: String,
    /// Other information associated with the token.
    #[pyo3(get)]
    pub info: PyPada,
}

#[pymethods]
impl PyToken {
    fn __repr__(&self) -> String {
        format!(
            "Token<(\"{}\", \"{}\", {:?})>",
            self.text, self.lemma, self.info
        )
    }
}

/// A Sanskrit parser.
#[pyclass(name = "Chedaka")]
pub struct PyChedaka {
    chedaka: Chedaka,
}

#[pymethods]
impl PyChedaka {
    /// Initialize a Sanskrit parser by reading the necessary data from the filesystem. This
    /// method raises a ValueError if the initialiation fails.
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

    /// Parses the given SLP1 input and returns a list of `Token` objects.
    pub fn run(&self, slp1_text: &str) -> Vec<PyToken> {
        let tokens = self.chedaka.run(slp1_text);
        let mut ret = Vec::new();

        for token in tokens {
            ret.push(PyToken {
                text: token.text.clone(),
                lemma: token.lemma(),
                info: token.info.into(),
            });
        }

        ret
    }
}
