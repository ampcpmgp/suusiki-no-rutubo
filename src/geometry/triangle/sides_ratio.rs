use super::sides;

pub struct SidesRatio {
    a: f64,
    b: f64,
    c: f64,
}

impl SidesRatio {
    pub fn new(a: f64, b: f64, c: f64) -> SidesRatio {
        SidesRatio { a, b, c }
    }

    pub fn is_valid(&self) -> bool {
        sides::is_valid(self.a, self.b, self.c)
    }

    pub fn a(&self) -> f64 {
        self.a
    }
    pub fn b(&self) -> f64 {
        self.b
    }
    pub fn c(&self) -> f64 {
        self.c
    }
}
