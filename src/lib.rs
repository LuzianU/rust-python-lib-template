mod calculator;
mod py;
pub use calculator::*;
use pyo3::prelude::*;

#[pymodule]
fn rust_python_lib_template(py: Python, m: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "py")?;
    child_module.add_class::<py::Calculator>()?;
    m.add_submodule(child_module)?;
    Ok(())
}
