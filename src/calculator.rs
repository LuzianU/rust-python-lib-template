pub struct Calculator {}

impl Calculator {
    pub fn new() -> Self {
        Calculator {}
    }

    pub fn sum(&self, a: usize, b: usize) -> usize {
        a + b
    }
}
