/*!
Wrappers for vidyut-prakriya arguments.

Pyo3 doesn't allow us to annotate existing enums, and using a wrapping struct has poor ergonomics
for callers. So instead, redefine our enums of interest.
*/
use pyo3::prelude::*;
use vidyut_prakriya::args as rust;

#[pyclass]
#[derive(Copy, Clone)]
pub enum Prayoga {
    Kartari,
    Karmani,
    Bhave,
}

impl From<Prayoga> for rust::Prayoga {
    fn from(val: Prayoga) -> Self {
        use Prayoga::*;
        match val {
            Kartari => Self::Kartari,
            Karmani => Self::Karmani,
            Bhave => Self::Bhave,
        }
    }
}

#[pyclass]
#[derive(Copy, Clone)]
pub enum Purusha {
    Prathama,
    Madhyama,
    Uttama,
}

impl From<Purusha> for rust::Purusha {
    fn from(val: Purusha) -> Self {
        use Purusha::*;
        match val {
            Prathama => Self::Prathama,
            Madhyama => Self::Madhyama,
            Uttama => Self::Uttama,
        }
    }
}

#[pyclass]
#[derive(Copy, Clone)]
pub enum Vacana {
    Eka,
    Dvi,
    Bahu,
}

impl From<Vacana> for rust::Vacana {
    fn from(val: Vacana) -> Self {
        use Vacana::*;
        match val {
            Eka => Self::Eka,
            Dvi => Self::Dvi,
            Bahu => Self::Bahu,
        }
    }
}

#[pyclass]
#[derive(Copy, Clone)]
pub enum Lakara {
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

impl From<Lakara> for rust::Lakara {
    fn from(val: Lakara) -> Self {
        use Lakara::*;
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

#[pyclass]
#[derive(Copy, Clone)]
pub enum Sanadi {
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

impl From<Sanadi> for rust::Sanadi {
    fn from(val: Sanadi) -> Self {
        use Sanadi::*;
        match val {
            San => Self::San,
            Yan => Self::Yan,
            Nic => Self::Nic,
        }
    }
}

#[pyclass]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Krt {
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

impl From<Krt> for rust::Krt {
    fn from(val: Krt) -> Self {
        use Krt::*;
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
