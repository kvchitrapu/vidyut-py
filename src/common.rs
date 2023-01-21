use pyo3::basic::CompareOp;
use pyo3::prelude::*;

use vidyut_kosha::semantics::*;

// We can't use `From<T> for Option<PyT>` because `Option` is not defined in this crate.
trait ToPy<T> {
    fn to_py(self) -> T;
}

// We can't use `From<Option<PyT>> for T` because `T` is not defined in this crate.
trait ToRust<T> {
    fn to_rust(self) -> T;
}

#[pyclass(name = "PartOfSpeech")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PyPartOfSpeech {
    Avyaya,
    Subanta,
    Tinanta,
}

#[pyclass(name = "Purusha")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PyPurusha {
    Prathama,
    Madhyama,
    Uttama,
}

impl ToPy<Option<PyPurusha>> for Purusha {
    fn to_py(self) -> Option<PyPurusha> {
        match self {
            Purusha::Prathama => Some(PyPurusha::Prathama),
            Purusha::Madhyama => Some(PyPurusha::Madhyama),
            Purusha::Uttama => Some(PyPurusha::Uttama),
            Purusha::None => None,
        }
    }
}

impl ToRust<Purusha> for Option<PyPurusha> {
    fn to_rust(self) -> Purusha {
        use Purusha as R;
        use PyPurusha as Py;
        match self {
            Some(x) => match x {
                Py::Prathama => R::Prathama,
                Py::Madhyama => R::Madhyama,
                Py::Uttama => R::Uttama,
            },
            None => R::None,
        }
    }
}

#[pyclass(name = "Linga")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PyLinga {
    Pum,
    Stri,
    Napumsaka,
}

impl ToPy<Option<PyLinga>> for Linga {
    fn to_py(self) -> Option<PyLinga> {
        match self {
            Linga::Pum => Some(PyLinga::Pum),
            Linga::Stri => Some(PyLinga::Stri),
            Linga::Napumsaka => Some(PyLinga::Napumsaka),
            Linga::None => None,
        }
    }
}

impl ToRust<Linga> for Option<PyLinga> {
    fn to_rust(self) -> Linga {
        use Linga as R;
        use PyLinga as Py;
        match self {
            Some(x) => match x {
                Py::Pum => R::Pum,
                Py::Stri => R::Stri,
                Py::Napumsaka => R::Napumsaka,
            },
            None => R::None,
        }
    }
}

#[pyclass(name = "Vacana")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PyVacana {
    Eka,
    Dvi,
    Bahu,
}

impl ToPy<Option<PyVacana>> for Vacana {
    fn to_py(self) -> Option<PyVacana> {
        match self {
            Vacana::Eka => Some(PyVacana::Eka),
            Vacana::Dvi => Some(PyVacana::Dvi),
            Vacana::Bahu => Some(PyVacana::Bahu),
            Vacana::None => None,
        }
    }
}

impl ToRust<Vacana> for Option<PyVacana> {
    fn to_rust(self) -> Vacana {
        use PyVacana as Py;
        use Vacana as R;
        match self {
            Some(x) => match x {
                Py::Eka => R::Eka,
                Py::Dvi => R::Dvi,
                Py::Bahu => R::Bahu,
            },
            None => R::None,
        }
    }
}

#[pyclass(name = "Vibhakti")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PyVibhakti {
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    Sambodhana,
}

impl ToPy<Option<PyVibhakti>> for Vibhakti {
    fn to_py(self) -> Option<PyVibhakti> {
        match self {
            Vibhakti::V1 => Some(PyVibhakti::V1),
            Vibhakti::V2 => Some(PyVibhakti::V2),
            Vibhakti::V3 => Some(PyVibhakti::V3),
            Vibhakti::V4 => Some(PyVibhakti::V4),
            Vibhakti::V5 => Some(PyVibhakti::V5),
            Vibhakti::V6 => Some(PyVibhakti::V6),
            Vibhakti::V7 => Some(PyVibhakti::V7),
            Vibhakti::Sambodhana => Some(PyVibhakti::Sambodhana),
            Vibhakti::None => None,
        }
    }
}

impl ToRust<Vibhakti> for Option<PyVibhakti> {
    fn to_rust(self) -> Vibhakti {
        use PyVibhakti as Py;
        use Vibhakti as R;

        match self {
            Some(x) => match x {
                Py::V1 => R::V1,
                Py::V2 => R::V2,
                Py::V3 => R::V3,
                Py::V4 => R::V4,
                Py::V5 => R::V5,
                Py::V6 => R::V6,
                Py::V7 => R::V7,
                Py::Sambodhana => R::Sambodhana,
            },
            None => R::None,
        }
    }
}

#[pyclass(name = "Lakara")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PyLakara {
    Lat,
    Lit,
    Lut,
    Lrt,
    Let,
    Lot,
    Lan,
    LinVidhi,
    LinAshih,
    Lun,
    LunNoAgama,
    Lrn,
}

impl ToPy<Option<PyLakara>> for Lakara {
    fn to_py(self) -> Option<PyLakara> {
        use PyLakara::*;
        match self {
            Lakara::Lat => Some(Lat),
            Lakara::Lit => Some(Lit),
            Lakara::Lut => Some(Lut),
            Lakara::Lrt => Some(Lot),
            Lakara::Let => Some(Let),
            Lakara::Lot => Some(Lot),
            Lakara::Lan => Some(Lan),
            Lakara::LinVidhi => Some(LinVidhi),
            Lakara::LinAshih => Some(LinAshih),
            Lakara::Lun => Some(Lun),
            Lakara::LunNoAgama => Some(LunNoAgama),
            Lakara::Lrn => Some(Lrn),
            Lakara::None => None,
        }
    }
}

impl ToRust<Lakara> for Option<PyLakara> {
    fn to_rust(self) -> Lakara {
        use Lakara as R;
        use PyLakara as Py;

        match self {
            Some(x) => match x {
                Py::Lat => R::Lat,
                Py::Lit => R::Lit,
                Py::Lut => R::Lut,
                Py::Lrt => R::Lrt,
                Py::Let => R::Let,
                Py::Lot => R::Lot,
                Py::Lan => R::Lan,
                Py::LinVidhi => R::LinVidhi,
                Py::LinAshih => R::LinAshih,
                Py::Lun => R::Lun,
                Py::LunNoAgama => R::LunNoAgama,
                Py::Lrn => R::Lrn,
            },
            None => R::None,
        }
    }
}

#[pyclass(name = "PadaPrayoga")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PyPadaPrayoga {
    Parasmaipada,
    AtmanepadaKartari,
    AtmanepadaNotKartari,
}

impl ToPy<Option<PyPadaPrayoga>> for PadaPrayoga {
    fn to_py(self) -> Option<PyPadaPrayoga> {
        use PyPadaPrayoga::*;
        match self {
            PadaPrayoga::Parasmaipada => Some(Parasmaipada),
            PadaPrayoga::AtmanepadaKartari => Some(AtmanepadaKartari),
            PadaPrayoga::AtmanepadaNotKartari => Some(AtmanepadaNotKartari),
            PadaPrayoga::None => None,
        }
    }
}

impl ToRust<PadaPrayoga> for Option<PyPadaPrayoga> {
    fn to_rust(self) -> PadaPrayoga {
        use PadaPrayoga as R;
        use PyPadaPrayoga as Py;

        match self {
            Some(x) => match x {
                Py::Parasmaipada => R::Parasmaipada,
                Py::AtmanepadaKartari => R::AtmanepadaKartari,
                Py::AtmanepadaNotKartari => R::AtmanepadaNotKartari,
            },
            None => R::None,
        }
    }
}

#[pyclass(name = "Dhatu")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PyDhatu {
    #[pyo3(get)]
    text: String,
}

#[pymethods]
impl PyDhatu {
    /// Creates a new `Pada`.
    #[new]
    #[args(py_args = "*", text)]
    fn new(text: String) -> Self {
        Self { text }
    }

    fn __repr__(&self) -> String {
        format!("Dhatu(text='{}')", self.text)
    }

    fn __richcmp__(&self, other: PyRef<PyDhatu>, op: CompareOp) -> Py<PyAny> {
        println!("Comparing dhatu");

        let py = other.py();
        let is_eq = self.text == other.text;

        match op {
            CompareOp::Eq => (is_eq).into_py(py),
            CompareOp::Ne => (!is_eq).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __str__(&self) -> String {
        self.text.clone()
    }
}

impl ToPy<PyDhatu> for Dhatu {
    fn to_py(self) -> PyDhatu {
        PyDhatu { text: self.0 }
    }
}

impl ToRust<Dhatu> for PyDhatu {
    fn to_rust(self) -> Dhatu {
        Dhatu(self.text)
    }
}

#[pyclass(name = "Pratipadika")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PyPratipadika {
    #[pyo3(get)]
    text: String,
    // dhatu: Option<PyDhatu>,
    // lingas: Vec<PyLinga>
}

#[pymethods]
impl PyPratipadika {
    /// Creates a new `Pratipadika`.
    #[new]
    #[args(py_args = "*", text)]
    fn new(text: String) -> Self {
        Self { text }
    }

    fn __repr__(&self) -> String {
        format!("Pratipadika(text='{}')", self.text)
    }

    fn __richcmp__(&self, other: PyRef<PyPratipadika>, op: CompareOp) -> Py<PyAny> {
        let py = other.py();
        let is_eq = self.text == other.text;

        match op {
            CompareOp::Eq => (is_eq).into_py(py),
            CompareOp::Ne => (!is_eq).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    fn __str__(&self) -> String {
        self.text.clone()
    }
}

impl ToPy<PyPratipadika> for Dhatu {
    fn to_py(self) -> PyPratipadika {
        PyPratipadika { text: self.0 }
    }
}

impl ToRust<Pratipadika> for PyPratipadika {
    fn to_rust(self) -> Pratipadika {
        Pratipadika::Basic {
            text: self.text,
            lingas: vec![],
        }
    }
}

#[pyclass(name = "Pada")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PyPada {
    #[pyo3(get)]
    pos: Option<PyPartOfSpeech>,

    #[pyo3(get)]
    dhatu: Option<PyDhatu>,

    #[pyo3(get)]
    pratipadika: Option<PyPratipadika>,

    #[pyo3(get)]
    purusha: Option<PyPurusha>,

    #[pyo3(get)]
    lakara: Option<PyLakara>,

    #[pyo3(get)]
    pada_prayoga: Option<PyPadaPrayoga>,

    #[pyo3(get)]
    vacana: Option<PyVacana>,

    #[pyo3(get)]
    linga: Option<PyLinga>,

    #[pyo3(get)]
    vibhakti: Option<PyVibhakti>,

    #[pyo3(get)]
    is_purvapada: bool,
}

#[pymethods]
impl PyPada {
    /// Create a new *tinanta* (verb).
    #[staticmethod]
    #[args(py_args = "*", dhatu, purusha, lakara, vacana, pada_prayoga)]
    #[allow(clippy::too_many_arguments)]
    fn make_tinanta(
        dhatu: Option<PyDhatu>,
        purusha: Option<PyPurusha>,
        vacana: Option<PyVacana>,
        lakara: Option<PyLakara>,
        pada_prayoga: Option<PyPadaPrayoga>,
    ) -> Self {
        Self {
            pos: Some(PyPartOfSpeech::Tinanta),
            dhatu,
            purusha,
            lakara,
            vacana,
            pada_prayoga,
            ..Default::default()
        }
    }

    /// Create a new *subanta* (nominal).
    #[staticmethod]
    #[args(py_args = "*", pratipadika, linga, vibhakti, vacana)]
    #[allow(clippy::too_many_arguments)]
    fn make_subanta(
        pratipadika: Option<PyPratipadika>,
        linga: Option<PyLinga>,
        vibhakti: Option<PyVibhakti>,
        vacana: Option<PyVacana>,
    ) -> Self {
        Self {
            pos: Some(PyPartOfSpeech::Subanta),
            pratipadika,
            vacana,
            vibhakti,
            linga,
            ..Default::default()
        }
    }

    /// Create a new *avyaya* (indeclinable).
    #[staticmethod]
    #[args(py_args = "*", pratipadika, linga, vibhakti, vacana)]
    #[allow(clippy::too_many_arguments)]
    fn make_avyaya(pratipadika: Option<PyPratipadika>) -> Self {
        Self {
            pos: Some(PyPartOfSpeech::Avyaya),
            pratipadika,
            ..Default::default()
        }
    }

    fn __richcmp__(&self, other: PyRef<PyPada>, op: CompareOp) -> Py<PyAny> {
        println!("Comparing pada");

        let py = other.py();

        // TODO: this is messy and can easily become buggy.
        let is_eq = self.pos == other.pos
            /*
            && self.dhatu == other.dhatu;
            */
            && self.vacana == other.vacana
            && self.lakara == other.lakara
            && self.purusha == other.purusha
            && self.pada_prayoga == other.pada_prayoga
            && self.vibhakti == other.vibhakti
            && self.linga == other.linga
            && self.is_purvapada == other.is_purvapada;

        match op {
            CompareOp::Eq => (is_eq).into_py(py),
            CompareOp::Ne => (!is_eq).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

impl From<Pada> for PyPada {
    fn from(val: Pada) -> Self {
        let mut res = PyPada::default();
        match val {
            Pada::Avyaya(_) => {
                res.pos = Some(PyPartOfSpeech::Avyaya);
            }
            Pada::Subanta(s) => {
                res.pos = Some(PyPartOfSpeech::Subanta);
                res.linga = s.linga.to_py();
                res.vibhakti = s.vibhakti.to_py();
                res.vacana = s.vacana.to_py();
                res.is_purvapada = s.is_purvapada;
            }
            Pada::Tinanta(t) => {
                res.pos = Some(PyPartOfSpeech::Tinanta);
                res.lakara = t.lakara.to_py();
                res.pada_prayoga = t.pada.to_py();
                res.purusha = t.purusha.to_py();
                res.vacana = t.vacana.to_py();
            }
            Pada::None => {
                res.pos = None;
            }
        };

        res
    }
}

impl From<PyPada> for Pada {
    fn from(val: PyPada) -> Self {
        use PyPartOfSpeech as PyPOS;

        match val.pos {
            None => Self::None,
            Some(PyPOS::Tinanta) => Pada::Tinanta(Tinanta {
                dhatu: val.dhatu.map(|x| x.to_rust()).unwrap(),
                purusha: val.purusha.to_rust(),
                vacana: val.vacana.to_rust(),
                lakara: val.lakara.to_rust(),
                pada: val.pada_prayoga.to_rust(),
            }),

            Some(PyPOS::Subanta) => Pada::Subanta(Subanta {
                pratipadika: val.pratipadika.map(|x| x.to_rust()).unwrap(),
                vacana: val.vacana.to_rust(),
                linga: val.linga.to_rust(),
                vibhakti: val.vibhakti.to_rust(),
                is_purvapada: val.is_purvapada,
            }),

            Some(PyPOS::Avyaya) => Pada::Avyaya(Avyaya {
                pratipadika: val.pratipadika.map(|x| x.to_rust()).unwrap(),
            }),
        }
    }
}
