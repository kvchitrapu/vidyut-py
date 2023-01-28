/*!
This library defines our Rust-Python bridge.

Almost all of the heavy lifting is done by PyO3, which generates a native Python module with the
help of a few macros. Then, Maturin combines the code here with our Python code to create our final
package.


Structure
---------

The `vidyut` function below defines our native module. It uses symbols from the `cheda`, `kosha`,
and `prakriya` Rust modules, which define bindings for their corresponding Rust crates.

To handle name collisions, we create submodules with PyO3's `wrap_pymodule!` macro. One restriction
of these native submodules is that we cannot `import` from them:

```python
# This will fail
from foo.bar import *
```

However, we can still access symbols by attribute:

```python
# This will fail
from foo import bar
X = bar.X
```

We use this pattern extensively. For an example, see `vidyut/kosha.py`.


Conventions
-----------

`__repr__`
~~~~~~~~~~

Prefer returning a string that could be `eval`-ed:

> For many types, this function makes an attempt to return **a string that would yield an object with
the same value when passed to eval()**; otherwise, the representation is a string enclosed in angle
brackets that contains the name of the type of the object together with additional information
often including the name and address of the object.

-- Python docs (emphasis added)

Comments
~~~~~~~~

Rustdoc comments are used as-is to document their Python counterparts [1]. If a comment is on an
item wrapped in a PyO3 macro, prefer an imperative style ("Return" vs. "Returns"). Otherwise, use
the normal Rust style ("Returns" vs. "Return").

[1]: https://pyo3.rs/v0.18.0/module.html#documentation

*/

#![deny(clippy::panic)]
#![warn(clippy::unwrap_used)]

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

mod cheda;
mod kosha;
mod prakriya;
mod sandhi;

/// Defines the `vidyut.cheda` native module.
///
/// For usage examples, see `vidyut/cheda.py`.
#[pymodule]
#[pyo3(name = "cheda")]
fn py_cheda(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<cheda::PyChedaka>()?;
    m.add_class::<cheda::PyToken>()?;

    Ok(())
}

/// Defines the `vidyut.kosha` native module.
///
/// For usage examples, see `vidyut/kosha.py`.
#[pymodule]
#[pyo3(name = "kosha")]
fn py_kosha(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<kosha::Kosha>()?;
    m.add_class::<kosha::Builder>()?;

    // These symbols have name collisions with their similarly named counterparts in the `prakriya`
    // module. By wrapping these symbols in their own native module, we avoid the name collision
    // and can freely use both sets of symbols.
    //
    // (Note: these symbols are hard to unify. The symbols here model the *structure* of Sanskrit
    // words, and the symbols in `prakriya` model the *arguments* to the Ashtadhyayi. While these
    // two symbols have substantial overlap, they model Sanskrit words slightly differently and
    // have different use cases.)
    m.add_class::<kosha::semantics::PyDhatu>()?;
    m.add_class::<kosha::semantics::PyLakara>()?;
    m.add_class::<kosha::semantics::PyLinga>()?;
    m.add_class::<kosha::semantics::PyPada>()?;
    m.add_class::<kosha::semantics::PyPratipadika>()?;
    m.add_class::<kosha::semantics::PyPadaPrayoga>()?;
    m.add_class::<kosha::semantics::PyPartOfSpeech>()?;
    m.add_class::<kosha::semantics::PyPurusha>()?;
    m.add_class::<kosha::semantics::PyVacana>()?;
    m.add_class::<kosha::semantics::PyVibhakti>()?;

    Ok(())
}

/// Defines the `vidyut.prakriya` native module.
///
/// For usage examples, see `vidyut/prakriya.py`.
#[pymodule]
#[pyo3(name = "prakriya")]
fn py_prakriya(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<prakriya::PyAshtadhyayi>()?;
    m.add_class::<prakriya::PyDhatupatha>()?;
    m.add_class::<prakriya::PyPrakriya>()?;
    m.add_class::<prakriya::PyStep>()?;

    // For details on these symbols, see the comments in `py_kosha`.
    m.add_class::<prakriya::args::PyDhatu>()?;
    m.add_class::<prakriya::args::PyPratipadika>()?;
    m.add_class::<prakriya::args::PyKrt>()?;
    m.add_class::<prakriya::args::PyLakara>()?;
    m.add_class::<prakriya::args::PyLinga>()?;
    m.add_class::<prakriya::args::PyPrayoga>()?;
    m.add_class::<prakriya::args::PyPurusha>()?;
    m.add_class::<prakriya::args::PySanadi>()?;
    m.add_class::<prakriya::args::PyVacana>()?;
    m.add_class::<prakriya::args::PyVibhakti>()?;

    Ok(())
}

/// Defines the `vidyut.sandhi` native module.
///
/// For usage examples, see `vidyut/sandhi.py`.
#[pymodule]
#[pyo3(name = "sandhi")]
fn py_sandhi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<sandhi::PySplitter>()?;
    m.add_class::<sandhi::PySplit>()?;

    Ok(())
}

/// Defines the `vidyut` native module.
///
/// In our Python code, this module is made available as `vidyut.vidyut`. For usage examples, see
/// any of the files in the `vidyut` Python package, e.g. `vidyut/kosha.py`.
#[pymodule]
fn vidyut(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(py_cheda))?;
    m.add_wrapped(wrap_pymodule!(py_kosha))?;
    m.add_wrapped(wrap_pymodule!(py_prakriya))?;
    m.add_wrapped(wrap_pymodule!(py_sandhi))?;

    Ok(())
}
