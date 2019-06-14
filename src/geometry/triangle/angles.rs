//! ```
//! use suusiki_no_rutubo::geometry::triangle::{Angles};
//!
//! let angle = Angles::new(10.0, 20.0, 150.0);
//! assert_eq!(angle.is_valid(), true);
//! ```
use super::*;

pub struct Angles {
    ab: f64,
    bc: f64,
    ca: f64,
}

impl Angles {
    pub fn new(ab: f64, bc: f64, ca: f64) -> Angles {
        Angles { ab, bc, ca }
    }

    /// ```
    /// use suusiki_no_rutubo::geometry::triangle::{Angles};
    ///
    /// let remaining_angle = Angles::remaining_angle(10.0, 20.0);
    /// assert_eq!(remaining_angle, 150.0);
    /// ```
    pub fn remaining_angle(ab: f64, bc: f64) -> f64 {
        TOTAL_ANGLE - ab - bc
    }

    pub fn is_valid(self) -> bool {
        return Self::is_valid_angle(self.ab)
            && Self::is_valid_angle(self.bc)
            && Self::is_valid_angle(self.ca)
            && (self.ab + self.bc + self.ca) == TOTAL_ANGLE;
    }

    pub fn is_valid_angle(angle: f64) -> bool {
        match angle {
            angle if angle <= 0.0 => false,
            angle if angle >= TOTAL_ANGLE => false,
            _ => true,
        }
    }

    pub fn ab(self) -> f64 {
        self.ab
    }
    pub fn bc(self) -> f64 {
        self.bc
    }
    pub fn ca(self) -> f64 {
        self.ca
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let remaining_angle = Angles::remaining_angle(10.0, 20.0);
        assert_eq!(remaining_angle, 150.0);

        let angle = Angles::new(10.0, 20.0, remaining_angle);
        assert_eq!(angle.is_valid(), true);
    }
}
