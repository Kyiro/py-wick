from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="py_wick",
    version="0.1.0",
    classifiers=[],
    packages=["py_wick"],
    rust_extensions=[RustExtension("py_wick.py_wick", "Cargo.toml", debug=False)],
    include_package_data=True,
    zip_safe=False,
)
