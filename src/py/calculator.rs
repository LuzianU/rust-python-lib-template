use pyo3::prelude::*;
#[pyclass]
pub struct Calculator {
    inner: crate::Calculator,
}

#[pymethods]
impl Calculator {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(Calculator {
            inner: crate::Calculator::new(),
        })
    }

    pub fn sum(&self, a: usize, b: usize) -> usize {
        self.inner.sum(a, b)
    }
}
