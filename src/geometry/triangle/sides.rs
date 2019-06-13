pub struct Sides {
    a: f64,
    b: f64,
    c: f64,
}

impl Sides {
    /// ```
    /// use suusiki_no_rutubo::geometry::triangle::{Sides};
    ///
    /// let sides = Sides::new(3.0, 4.0, 5.0);
    /// assert_eq!(sides.is_valid(), true);
    ///
    /// let sides = Sides::new(1.0, 1.0, 3.0);
    /// assert_eq!(sides.is_valid(), false);
    /// ```
    pub fn new(a: f64, b: f64, c: f64) -> Sides {
        Sides { a, b, c }
    }

    pub fn is_valid(self) -> bool {
        return self.a > 0.0
            && self.b > 0.0
            && self.c > 0.0
            // 参考: https://www.geeksforgeeks.org/check-whether-triangle-valid-not-sides-given/
            && (self.a + self.b) > self.c
            && (self.b + self.c) > self.a
            && (self.c + self.a) > self.b;
    }

    pub fn a(self) -> f64 {
        self.a
    }

    pub fn b(self) -> f64 {
        self.b
    }

    pub fn c(self) -> f64 {
        self.c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sides = Sides::new(3.0, 4.0, 5.0);
        assert_eq!(sides.is_valid(), true);

        let sides = Sides::new(1.0, 1.0, 3.0);
        assert_eq!(sides.is_valid(), false);
    }
}
