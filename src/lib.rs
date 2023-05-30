use pyo3::prelude::*;
mod py;
pub struct Calculator {}

impl Calculator {
    pub fn new() -> Self {
        Calculator {}
    }

    pub fn sum(&self, a: usize, b: usize) -> usize {
        a + b
    }
}

#[pymodule]
fn rust_python_lib_template(py: Python, m: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "py")?;
    child_module.add_class::<py::Calculator>()?;
    m.add_submodule(child_module)?;
    Ok(())
}
