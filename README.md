# signal-groups

## Developer Getting Started

You will need both [Rust](https://rustup.rs/) and Python 3.7+ installed on your system.
To install the project in your virtualenv:

```sh
pip install -r requirements.txt
python setup.py develop
```

Then run the tests via `pytest -v tests/` to confirm all is working.
Tests are (mostly) ported to Python from the upstream crate.
You can use the tests as a reference for how to use the library (start with the integration tests).

When developing, simply run `python setup.py develop` as you make changes to rebuild the library.
This script will handle compilation on the Rust side.

# Building wheels

See instructions [here](https://github.com/PyO3/setuptools-rust#binary-wheels-on-linux). In brief:

```
docker pull quay.io/pypa/manylinux2014_x86_64
docker run --rm -v `pwd`:/io quay.io/pypa/manylinux2014_x86_64 /io/build-wheels.sh
```
