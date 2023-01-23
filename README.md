<div align="center">
<h1><code>vidyut-py</code></h1>
<p><i>Python bindings for Vidyut</i></p>
</div>

`vidyut-py` defines Python bindings for [Vidyut][vidyut], a high-performance
Sanskrit toolkit. These bindings are under active development as part of the
[Ambuda][ambuda] project.


Installing
----------

Install and update with [pip](https://pip.pypa.io/en/stable/getting-started/):

    $ pip install -U vidyut

Currently, `vidyut-py` does not include any linguistic data. For now, you must
build this linguistic data manually. Doing so requires the `cargo` command.

To build this data, run the following commands:

    git clone git@github.com:ambuda-org/vidyut.git
    cd vidyut/vidyut-cheda
    make install

The output data will be in `data/vidyut-x.y.z`, where `x.y.z` is the Vidyut
version. Once the `data` folder has been created, you can move it wherever you
like.


Links
-----

For users:

- Documentation: https://vidyut.readthedocs.io
- Releases: https://pypi.org/project/vidyut/
- Changes: https://vidyut.readthedocs.io/en/latest/changes.html
- Source Code: https://github.com/ambuda-org/vidyut
- Issues: https://github.com/ambuda-org/vidyut/issues

For contributors:

- Contributing: [CONTRIBUTING.md](CONTRIBUTING.md)

For other questions:

- Chat: https://discord.gg/7rGdTyWY7Z
