use super::sides;

pub struct SidesRatio {
    a: f32,
    b: f32,
    c: f32,
}

impl SidesRatio {
    pub fn new(a: f32, b: f32, c: f32) -> SidesRatio {
        SidesRatio { a, b, c }
    }

    pub fn is_valid(&self) -> bool {
        sides::is_valid(self.a, self.b, self.c)
    }

    pub fn a(&self) -> f32 {
        self.a
    }
    pub fn b(&self) -> f32 {
        self.b
    }
    pub fn c(&self) -> f32 {
        self.c
    }
}
