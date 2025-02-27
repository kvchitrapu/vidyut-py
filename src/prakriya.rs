use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyFileNotFoundError, PyKeyError, PyValueError};
use pyo3::prelude::*;
use std::path::PathBuf;
use vidyut_prakriya::args::{Dhatu, Gana, KrdantaArgs, SubantaArgs, TinantaArgs};
use vidyut_prakriya::{Ashtadhyayi, Dhatupatha};
use vidyut_prakriya::{Prakriya, Step};

pub mod args;
use args::*;

/// A step in the derivation.
#[pyclass(name = "Step", get_all)]
#[derive(Clone)]
pub struct PyStep {
    /// The rule that was applied.
    pub rule: String,
    /// The result of applying `rule`.
    pub result: String,
}

#[pymethods]
impl PyStep {
    #[new]
    fn new(rule: String, result: String) -> Self {
        Self { rule, result }
    }

    fn __repr__(&self) -> String {
        format!("Step(rule='{}', result='{}')", self.rule, self.result)
    }

    fn __richcmp__(&self, other: PyRef<PyStep>, op: CompareOp) -> Py<PyAny> {
        let py = other.py();
        let is_eq = self.rule == other.rule && self.result == other.result;

        match op {
            CompareOp::Eq => (is_eq).into_py(py),
            CompareOp::Ne => (!is_eq).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

/// A derivation.
#[pyclass(name = "Prakriya", get_all)]
pub struct PyPrakriya {
    /// The final output of the derivation.
    pub text: String,
    /// All of the steps that were applied during the derivation.
    pub history: Vec<PyStep>,
}

fn to_py_history(history: &[Step]) -> Vec<PyStep> {
    history
        .iter()
        .map(|x| PyStep {
            rule: x.rule().to_string(),
            result: x.result().clone(),
        })
        .collect()
}

fn to_py_prakriyas(prakriyas: Vec<Prakriya>) -> Vec<PyPrakriya> {
    prakriyas
        .into_iter()
        .map(|p| PyPrakriya {
            text: String::from(p.text()),
            history: to_py_history(p.history()),
        })
        .collect()
}

/// Provides an interface to the Dhatupatha.
#[pyclass(name = "Dhatupatha")]
pub struct PyDhatupatha(Dhatupatha);

#[pymethods]
impl PyDhatupatha {
    /// Create a new dhatupatha instance from the given `path`.
    ///
    /// `path` should point to a 3-column TSV with columns `code`, `dhatu`, and `artha`.
    ///
    /// - `code` should be a number in the format "X.Y", such as "01.0001".
    ///
    /// - `dhatu` should be the dhatu`s aupadeshika form transliterated as SLP1.
    ///   Svaras and nasal vowels must be indicated explictly.
    ///
    /// - `artha` is an arbitrary string.
    ///
    /// Exceptions:
    /// - `FlieNotFoundError` if the file does not exist.
    /// - `ValueError` if the function cannot parse the input file.
    #[new]
    pub fn new(path: PathBuf) -> PyResult<Self> {
        if !path.exists() {
            let message = format!("No such file: '{}'", path.display());
            return Err(PyFileNotFoundError::new_err(message));
        }

        match Dhatupatha::from_path(&path) {
            Ok(d) => Ok(Self(d)),
            Err(e) => {
                let message = format!(
                    "Could not parse file '{}'. Error was: `{}`",
                    path.display(),
                    e
                );
                Err(PyValueError::new_err(message))
            }
        }
    }

    /// Return the dhatu with the given `code`, if it exists.
    ///
    /// If a dhatu is found, it will be returned by value.
    ///
    /// Exceptions:
    /// - `KeyError` if the given `code` is not found.
    pub fn __getitem__(&self, code: String) -> PyResult<PyDhatu> {
        match self.0.get(&code) {
            Some(d) => Ok(PyDhatu::new(d.clone())),
            None => Err(PyKeyError::new_err(code)),
        }
    }
}

/// An interface to the rules of the Ashtadhyayi.
#[pyclass(name = "Ashtadhyayi")]
#[derive(Default)]
pub struct PyAshtadhyayi(Ashtadhyayi);

#[pymethods]
impl PyAshtadhyayi {
    /// Create an interface with sane defaults.
    #[new]
    pub fn new() -> Self {
        Self(Ashtadhyayi::new())
    }

    /// Return all possible tinanta prakriyas that can be derived with the given initial
    /// conditions.
    #[pyo3(signature = (*, dhatu, prayoga, purusha, vacana, lakara, sanadi = None))]
    pub fn derive_tinantas(
        &self,
        dhatu: &PyDhatu,
        prayoga: PyPrayoga,
        purusha: PyPurusha,
        vacana: PyVacana,
        lakara: PyLakara,
        sanadi: Option<PySanadi>,
    ) -> Vec<PyPrakriya> {
        let tin_args = TinantaArgs::builder()
            .prayoga(prayoga.into())
            .purusha(purusha.into())
            .vacana(vacana.into())
            .lakara(lakara.into())
            .build()
            .expect("should have all required fields");

        let mut dhatu = Dhatu::builder()
            .upadesha(&dhatu.upadesha())
            // TODO: set gana
            .gana(Gana::Bhvadi);

        if let Some(sanadi) = sanadi {
            dhatu = dhatu.sanadi(&[sanadi.into()]);
        }
        let dhatu = dhatu.build().expect("should have all required fields");

        let results = self.0.derive_tinantas(&dhatu, &tin_args);
        to_py_prakriyas(results)
    }

    /// Return all possible krdanta prakriyas that can be derived with the given initial
    /// conditions.
    #[pyo3(signature = (*, dhatu, krt))]
    pub fn derive_krdantas(&self, dhatu: &PyDhatu, krt: PyKrt) -> Vec<PyPrakriya> {
        let args = KrdantaArgs::builder()
            .krt(krt.into())
            .build()
            .expect("should have all required fields");

        let dhatu = dhatu.as_ref();
        let results = self.0.derive_krdantas(dhatu, &args);
        to_py_prakriyas(results)
    }

    /// Return all possible krdanta prakriyas that can be derived with the given initial
    /// conditions.
    #[pyo3(signature = (*, pratipadika, linga, vibhakti, vacana))]
    pub fn derive_subantas(
        &self,
        pratipadika: &PyPratipadika,
        linga: PyLinga,
        vibhakti: PyVibhakti,
        vacana: PyVacana,
    ) -> Vec<PyPrakriya> {
        let args = SubantaArgs::builder()
            .linga(linga.into())
            .vibhakti(vibhakti.into())
            .vacana(vacana.into())
            .build()
            .expect("should have all required fields");

        let pratipadika = pratipadika.as_ref();
        let results = self.0.derive_subantas(pratipadika, &args);
        to_py_prakriyas(results)
    }
}
