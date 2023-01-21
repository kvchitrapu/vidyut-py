#![doc = include_str!("../README.md")]
#![deny(clippy::panic)]
#![warn(clippy::unwrap_used)]

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

mod cheda;
mod common;
mod kosha;
mod prakriya;

#[pymodule]
#[pyo3(name = "kosha")]
fn py_kosha(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<kosha::Kosha>()?;
    m.add_class::<kosha::Builder>()?;
    m.add_class::<common::PyDhatu>()?;
    m.add_class::<common::PyLakara>()?;
    m.add_class::<common::PyLinga>()?;
    m.add_class::<common::PyPada>()?;
    m.add_class::<common::PyPratipadika>()?;
    m.add_class::<common::PyPadaPrayoga>()?;
    m.add_class::<common::PyPartOfSpeech>()?;
    m.add_class::<common::PyPurusha>()?;
    m.add_class::<common::PyVacana>()?;
    m.add_class::<common::PyVibhakti>()?;

    Ok(())
}

#[pymodule]
#[pyo3(name = "prakriya")]
fn py_prakriya(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<kosha::Kosha>()?;
    m.add_class::<prakriya::PyAshtadhyayi>()?;
    m.add_class::<prakriya::PyDhatupatha>()?;
    m.add_class::<prakriya::args::Prayoga>()?;
    m.add_class::<prakriya::args::Purusha>()?;
    m.add_class::<prakriya::args::Vacana>()?;
    m.add_class::<prakriya::args::Lakara>()?;
    m.add_class::<prakriya::args::Sanadi>()?;
    m.add_class::<prakriya::args::Krt>()?;

    Ok(())
}

#[pymodule]
fn vidyut(_py: Python, m: &PyModule) -> PyResult<()> {
    // vidyut-cheda
    m.add_class::<cheda::PyChedaka>()?;

    m.add_wrapped(wrap_pymodule!(py_prakriya))?;
    m.add_wrapped(wrap_pymodule!(py_kosha))?;

    Ok(())
}
