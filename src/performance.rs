use std::time::Instant;

pub struct Performance {
    start: Instant,
}
impl Performance {
    pub fn new() -> Performance {
        Performance {
            start: Instant::now(),
        }
    }
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
    pub fn end(&self, message: &str) {
        let end = &self.start.elapsed();
        println!("{} > {:.2?}", message, end);
    }
}
