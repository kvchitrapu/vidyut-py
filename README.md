<div align="center">
<h1><code>vidyut-py</code></h1>
<p><i>Python bindings for Vidyut</i></p>
</div>

`vidyut-py` defines Python bindings for [Vidyut][vidyut], a high-performance
Sanskrit toolkit. These are the same bindings we use for our work on
[Ambuda][ambuda], which is written primarily in Python.

**Status**: In progress. This code is suitable for early adopters and
developers. Our API is mostly stable, but expect breaking changes when
upgrading.


- [Overview](#overview)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [Data](#data)

[vidyut]: https://github.com/ambuda-org/vidyut
[ambuda]: https://ambuda.org


Overview
--------

Vidyut, our high-performance Sanskrit toolkit, is implemented in Rust. Rust is
a wonderful language, but it is not right for all scenarios, and it is not a
language that many programmers know already. `vidyut-py` provides friendly
Python bindings on top of Rust so that you can use Vidyut more easily.

In general, our Python API is lightweight and mirrors the underlying Rust API,
with minor change to be more Pythonic.


Installation
------------

Our Python bindings are published under the `vidyut` package on PyPI and do not
require a Rust installation. You can install the `vidyut` package like so:

```python
$ pip install vidyut
```

You can experiment with our latest bindings by installing directly from this repo:

```python
$ pip install git+https://github.com/ambuda-org/vidyut-py.git
```

Usage
-----

Using `vidyut-cheda`:

```python
from vidyut.cheda import Chedaka

# For details on what this path should point to, see `Setup` below.
chedaka = Chedaka('/path/to/vidyut-data')

# All input must be in SLP1.
print(chedaka.run('gacCati'))
```

Using `vidyut-kosha`:

```python
from vidyut.kosha import Kosha

kosha = Kosha("/path/to/vidyut-data/kosha")
for result in kosha.get("gacCati"):
    print(result)
```

Using `vidyut-prakriya`:

```python
from vidyut.prakriya import Ashtadhyayi, Dhatupatha

d = Dhatupatha("/path/to/vidyut-data/prakriya/dhatupatha.tsv")
bhu = d["01.0001"]

prakriyas = a.derive_tinantas(
    dhatu=dhatu,
    prayoga=Prayoga.Kartari,
    purusha=Purusha.Prathama,
    vacana=Vacana.Eka,
    lakara=Lakara.Lat,
)

assert len(prakriyas) == 1
assert prakriyas[0].text == "Bavati"
```


Setup
-----

*(Requires Rust's `cargo` command)*

Currently, `vidyut-py` does not include any linguistic data. To use Vidyut, you
must build this linguistic data manually.

To build this data, please use the main [Vidyut][vidyut] repo as follows.

    # Build our linguistic data by using the main `vidyut` repo.
    git clone git@github.com:ambuda-org/vidyut.git
    cd vidyut/vidyut-cheda
    make install

    # The output data will be in `data/vidyut-x.y.z`, where `x.y.z` is the Vidyut version.
    # Once the `data` folder has been created, you can move it wherever you like.
    ls data/vidyut-0.1.0/

Then, you can pass the output path into `Chedaka`:

    chedaka = Chedaka('path/to/vidyut-0.1.0')
