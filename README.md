# Library Template for Python and Rust

This is a template repository for creating a library using Maturin and PyO3 that can be used in both Python and Rust projects.

## Getting Started

Set up a Python virtual environment:
```properties
python -m venv .env
```

Activate the virtual environment. For PowerShell:
```properties
.\.env\Scripts\Activate.ps1
```

Initialize the Rust bindings using Maturin (pyo3):
```properties
maturin init
```

Install the library into your virtual environment (make sure .env is activated)
```properties
maturin develop
```

You can now use the library in both Rust or Python (make sure .env is activated)

To automatically create a GitHub release including cross-compiled binaries, create a new Git tag for your release version:
```properties
git tag vX.X.X
git push origin vX.X.X
```

This will trigger a GitHub action that cross-compiles the Python wheels for your library and adds them to the created GitHub release with the corresponding tag.

This might require setting the GitHub workflow permissions to read and write.

## Learn More
- [Maturin documentation](https://www.maturin.rs/)
- [PyO3 documentation](https://pyo3.rs/)
