/*!
Wrappers for vidyut-prakriya arguments.

Pyo3 doesn't allow us to annotate existing enums, and using a wrapping struct has poor ergonomics
for callers. So instead, redefine our enums of interest.
*/
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use vidyut_prakriya::args::*;

/// A verb root.
#[pyclass(name = "Dhatu", module = "prakriya")]
pub struct PyDhatu(Dhatu);

impl PyDhatu {
    pub fn new(d: Dhatu) -> Self {
        Self(d)
    }
}

#[pymethods]
impl PyDhatu {
    /// The aupadeshika form of this dhatu.
    #[getter]
    pub fn upadesha(&self) -> String {
        self.0.upadesha().to_string()
    }

    fn __repr__(&self) -> String {
        format!("Dhatu(upadesha='{}')", self.0.upadesha())
    }
}

impl PyDhatu {
    pub fn as_ref(&self) -> &Dhatu {
        &self.0
    }
}

/// A nominal stem.
#[pyclass(name = "Pratipadika", module = "prakriya")]
pub struct PyPratipadika(Pratipadika);

impl PyPratipadika {
    pub fn as_ref(&self) -> &Pratipadika {
        &self.0
    }
}

#[pymethods]
impl PyPratipadika {
    /// Create a new pratipadika with the given text.
    #[new]
    #[pyo3(signature = (*, text))]
    pub fn new(text: String) -> Self {
        Self(Pratipadika::new(text))
    }

    /// The text of this stem.
    #[getter]
    pub fn text(&self) -> String {
        self.0.text().to_string()
    }

    fn __repr__(&self) -> String {
        format!("Pratipadika(text='{}')", self.0.text())
    }

    fn __richcmp__(&self, other: PyRef<PyPratipadika>, op: CompareOp) -> Py<PyAny> {
        let py = other.py();
        let is_eq = self.0.text() == other.0.text();

        match op {
            CompareOp::Eq => (is_eq).into_py(py),
            CompareOp::Ne => (!is_eq).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

#[pyclass(name = "Krt", module = "prakriya")]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum PyKrt {
    /// -Alu
    Aluc,
    /// -Aru
    Aru,
    /// -a
    Ga,
    /// -a
    GaY,
    /// -in
    GinuR,
    /// -ura
    Gurac,
    /// -a (Izatkara, duzkara, sukara, ...)
    Kal,
    /// -van
    Nvanip,
    /// -a
    Ra,
    /// -in
    Rini,
    /// -aka
    Rvuc,
    /// -aka
    Rvul,
    /// -ya
    Ryat,
    /// -ana
    Ryuw,
    /// -Ana (laBamAna, sevamAna, ...)
    SAnac,
    /// -Ana
    SAnan,
    /// -a
    Sa,
    /// -at (gacCat, Bavat, ...)
    Satf,
    /// -Taka (gATaka)
    Takan,
    /// -Tu (vepaTu). Allowed only for dhatus that are `qvit`.
    Tuc,
    /// -Uka
    Uka,
    /// -a
    aR,
    /// -a
    ac,
    /// -anIya (gamanIya, BavanIya, ...)
    anIyar,
    /// -ani
    ani,
    /// -at (jarat)
    atfn,
    /// -Ana
    cAnaS,
    /// -in
    ini,
    /// -itra
    itra,
    /// -izRu (alaMkarizRu, prajanizRu, ...)
    izRuc,
    /// -a
    ka,
    /// -i (udaDi, ...)
    ki,
    /// -i
    kin,
    /// -luka (BIluka)
    klukan,
    /// -mara
    kmarac,
    /// -nu
    knu,
    /// -ru (BIru)
    kru,
    /// -ruka (BIruka)
    kruka,
    /// -snu (glAsnu, jizRu, ...)
    ksnu,
    /// -ta (gata, bhUta, ...)
    kta,
    /// -tavat (gatavat, bhUtavat, ...)
    ktavatu,
    /// -ti
    ktic,
    /// -ti
    ktin,
    /// -tri
    ktri,
    /// -tvA (gatvA, bhUtva, ...)
    ktvA,
    /// -ura (BaNgura, ...)
    kurac,
    /// -vara
    kvarap,
    /// -vas
    kvasu,
    /// (empty suffix)
    kvip,
    /// -ya
    kyap,
    /// -ana
    lyu,
    /// -ana
    lyuw,
    /// -na
    naN,
    /// -naj
    najiN,
    /// -na (svapna)
    nan,
    /// -u
    qu,
    /// -na (namra, kampra, ...)
    ra,
    /// -ru
    ru,
    /// -tavya (gantavya, bhavitavya, ...)
    tavya,
    /// -tavya
    tavyat,
    /// -tf (gantA, bhavitA, ...)
    tfc,
    /// -tf
    tfn,
    /// -tum (gantum, bhavitum, ...)
    tumun,
    /// -u (yuyutsu, Bikzu, ...)
    u,
    /// -uka
    ukaY,
    /// -vaca
    varac,
    /// -aka
    vuY,
    /// -aka
    vun,
    /// -a
    wak,
    /// -ya
    yat,
    /// -ana
    yuc,
    /// -Aka
    zAkan,
    /// -aka
    zvun,
    /// -tra
    zwran,
}

impl From<PyKrt> for Krt {
    fn from(val: PyKrt) -> Self {
        use PyKrt::*;
        match val {
            Aluc => Self::Aluc,
            Aru => Self::Aru,
            Ga => Self::Ga,
            GaY => Self::GaY,
            GinuR => Self::GinuR,
            Gurac => Self::Gurac,
            Kal => Self::Kal,
            Nvanip => Self::Nvanip,
            Ra => Self::Ra,
            Rini => Self::Rini,
            Rvuc => Self::Rvuc,
            Rvul => Self::Rvul,
            Ryat => Self::Ryat,
            Ryuw => Self::Ryuw,
            SAnac => Self::SAnac,
            SAnan => Self::SAnan,
            Sa => Self::Sa,
            Satf => Self::Satf,
            Takan => Self::Takan,
            Tuc => Self::Tuc,
            Uka => Self::Uka,
            aR => Self::aR,
            ac => Self::ac,
            anIyar => Self::anIyar,
            ani => Self::ani,
            atfn => Self::atfn,
            cAnaS => Self::cAnaS,
            ini => Self::ini,
            itra => Self::itra,
            izRuc => Self::izRuc,
            ka => Self::ka,
            // TODO: kAnac => Self::kAnac,
            ki => Self::ki,
            kin => Self::kin,
            klukan => Self::klukan,
            kmarac => Self::kmarac,
            knu => Self::knu,
            kru => Self::kru,
            kruka => Self::kruka,
            ksnu => Self::ksnu,
            kta => Self::kta,
            ktavatu => Self::ktavatu,
            ktic => Self::ktic,
            ktin => Self::ktin,
            ktri => Self::ktri,
            ktvA => Self::ktvA,
            kurac => Self::kurac,
            kvarap => Self::kvarap,
            kvasu => Self::kvasu,
            kvip => Self::kvip,
            kyap => Self::kyap,
            lyu => Self::lyu,
            lyuw => Self::lyuw,
            naN => Self::naN,
            najiN => Self::najiN,
            nan => Self::nan,
            qu => Self::qu,
            ra => Self::ra,
            ru => Self::ru,
            tavya => Self::tavya,
            tavyat => Self::tavyat,
            tfc => Self::tfc,
            tfn => Self::tfn,
            tumun => Self::tumun,
            u => Self::u,
            ukaY => Self::ukaY,
            varac => Self::varac,
            vuY => Self::vuY,
            vun => Self::vun,
            wak => Self::wak,
            yat => Self::yat,
            yuc => Self::yuc,
            zAkan => Self::zAkan,
            zvun => Self::zvun,
            zwran => Self::zwran,
        }
    }
}

/// The lakara to use in the derivation.
#[pyclass(name = "Lakara", module = "prakriya")]
#[derive(Copy, Clone)]
pub enum PyLakara {
    /// Describes action in the present tense. Ssometimes called the *present indicative*.
    Lat,
    /// Describes unwitnessed past action. Sometimes called the *perfect*.
    Lit,
    /// Describes future action after the current day. Sometimes called the *periphrastic future*.
    Lut,
    /// Describes general future action. Sometimes called the *simple future*.
    Lrt,
    /// The Vedic subjunctive. `vidyut-prakriya` currently has poor support for this lakara.
    Let,
    /// Describes commands. Sometimes called the *imperative*.
    Lot,
    /// Describes past action before the current day. Sometimes called the *imperfect*.
    Lan,
    /// Describes potential or hypothetical actions. Sometimes called the *optative*.
    VidhiLin,
    /// Describes wishes and prayers. Sometimes called the *benedictive*.
    AshirLin,
    /// Describes general past action. Sometimes called the *aorist*.
    Lun,
    /// Describes past counterfactuals ("would not have ..."). Sometimes called the *conditional*.
    Lrn,
}

impl From<PyLakara> for Lakara {
    fn from(val: PyLakara) -> Self {
        use PyLakara::*;
        match val {
            Lat => Self::Lat,
            Lit => Self::Lit,
            Lut => Self::Lut,
            Lrt => Self::Lrt,
            Let => Self::Let,
            Lot => Self::Lot,
            Lan => Self::Lan,
            VidhiLin => Self::VidhiLin,
            AshirLin => Self::AshirLin,
            Lun => Self::Lun,
            Lrn => Self::Lrn,
        }
    }
}

/// The linga to use in the derivation.
#[pyclass(name = "Linga", module = "prakriya")]
#[derive(Copy, Clone)]
pub enum PyLinga {
    /// The masculine.
    Pum,
    /// The feminine.
    Stri,
    /// The neuter.
    Napumsaka,
}

impl From<PyLinga> for Linga {
    fn from(val: PyLinga) -> Self {
        use PyLinga::*;
        match val {
            Pum => Self::Pum,
            Stri => Self::Stri,
            Napumsaka => Self::Napumsaka,
        }
    }
}

/// The prayoga of some tinanta.
#[pyclass(name = "Prayoga", module = "prakriya")]
#[derive(Copy, Clone)]
pub enum PyPrayoga {
    /// Usage coreferent with the agent, e.g. "The horse *goes* to the village."
    Kartari,
    /// Usage coreferent with the object, e.g. "The village *is gone to* by the horse."
    Karmani,
    /// Usage without a referent, e.g. "*There is motion* by the horse to the village."
    /// bhAve prayoga generally produces the same forms as karmani prayoga.
    Bhave,
}

impl From<PyPrayoga> for Prayoga {
    fn from(val: PyPrayoga) -> Self {
        use PyPrayoga::*;
        match val {
            Kartari => Self::Kartari,
            Karmani => Self::Karmani,
            Bhave => Self::Bhave,
        }
    }
}

/// The person of some tinanta.
#[pyclass(name = "Purusha", module = "prakriya")]
#[derive(Copy, Clone)]
pub enum PyPurusha {
    /// The third person.
    Prathama,
    /// The second person.
    Madhyama,
    /// The first person.
    Uttama,
}

impl From<PyPurusha> for Purusha {
    fn from(val: PyPurusha) -> Self {
        use PyPurusha::*;
        match val {
            Prathama => Self::Prathama,
            Madhyama => Self::Madhyama,
            Uttama => Self::Uttama,
        }
    }
}

#[pyclass(name = "Sanadi", module = "prakriya")]
#[derive(Copy, Clone)]
pub enum PySanadi {
    /// `san`, which creates desiderative roots per 3.1.7.
    ///
    /// Examples: buBUzati, ninIzati.
    San,
    /// `yaN`, which creates intensive roots per 3.1.22. For certain dhatus, the semantics are
    /// instead "crooked movement" (by 3.1.23) or "contemptible" action (by 3.1.24).
    ///
    /// Examples: boBUyate, nenIyate.
    ///
    /// Constraints: can be used only if the dhatu starts with a consonant and has exactly one
    /// vowel. If this constraint is violated, our APIs will return an `Error`.
    Yan,
    /// `Nic`, which creates causal roots per 3.1.26.
    ///
    /// Examples: BAvayati, nAyayati.
    Nic,
}

impl From<PySanadi> for Sanadi {
    fn from(val: PySanadi) -> Self {
        use PySanadi::*;
        match val {
            San => Self::San,
            Yan => Self::Yan,
            Nic => Self::Nic,
        }
    }
}

/// The number of some tinanta or subanta.
#[pyclass(name = "Vacana", module = "prakriya")]
#[derive(Copy, Clone)]
pub enum PyVacana {
    /// The singular.
    Eka,
    /// The dual.
    Dvi,
    /// The plural.
    Bahu,
}

impl From<PyVacana> for Vacana {
    fn from(val: PyVacana) -> Self {
        use PyVacana::*;
        match val {
            Eka => Self::Eka,
            Dvi => Self::Dvi,
            Bahu => Self::Bahu,
        }
    }
}

/// The case ending of some subanta.
#[pyclass(name = "Vibhakti", module = "prakriya")]
#[derive(Copy, Clone)]
pub enum PyVibhakti {
    /// The first vibhakti. Sometimes called the *nominative case*.
    Prathama,
    /// The second vibhakti. Sometimes called the *accusative case*.
    Dvitiya,
    /// The third vibhakti. Sometimes called the *instrumental case*.
    Trtiya,
    /// The fourth vibhakti. Sometimes called the *dative case*.
    Caturthi,
    /// The fifth vibhakti. Sometimes called the *ablative case*.
    Panchami,
    /// The sixth vibhakti. Sometimes called the *genitive case*.
    Sasthi,
    /// The seventh vibhakti. Sometimes called the *locative case*.
    Saptami,
    /// The first vibhakti used in the sense of *sambodhana*. Sometimes called the *vocative case*.
    ///
    /// *Sambodhana* is technically not a *vibhakti but rather an additional semantic condition
    /// that conditions the first vibhakti. But we felt that users would find it more convenient to
    /// have this condition available on `Vibhakti` directly rather than have to define the
    /// *sambodhana* condition separately.
    Sambodhana,
}

impl From<PyVibhakti> for Vibhakti {
    fn from(val: PyVibhakti) -> Self {
        use PyVibhakti::*;
        match val {
            Prathama => Self::Prathama,
            Dvitiya => Self::Dvitiya,
            Trtiya => Self::Trtiya,
            Caturthi => Self::Caturthi,
            Panchami => Self::Panchami,
            Sasthi => Self::Sasthi,
            Saptami => Self::Saptami,
            Sambodhana => Self::Sambodhana,
        }
    }
}
