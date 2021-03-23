#!/usr/bin/env python

import sys
from setuptools import setup
from setuptools_rust import Binding, RustExtension

with open("README.md", "r") as fh:
    long_description = fh.read()

setup(
    name="signal-groups",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: GNU Affero General Public License v3 or later (AGPLv3+)",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    description="Rust extension providing Python bindings to the v2 signal groups",
    long_description=long_description,
    long_description_content_type="text/markdown",
    packages=["signal_groups"],
    rust_extensions=[
        RustExtension(
            'signal_groups.api',
            'Cargo.toml',
            binding=Binding.PyO3),
        RustExtension(
            'signal_groups.crypto',
            'Cargo.toml',
            binding=Binding.PyO3),
        RustExtension(
            'signal_groups.signal_groups',
            'Cargo.toml',
            binding=Binding.PyO3),
        ],
    setup_requires=["setuptools-rust>=0.10.1", "wheel"],
    zip_safe=False,  # Rust extensions are not zip safe
)
