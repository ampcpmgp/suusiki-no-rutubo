//! ```
//! use suusiki_no_rutubo::geometry::triangle::{Angles};
//!
//! let angles = Angles::new(10.0, 20.0, 150.0);
//! assert_eq!(angles.is_valid(), true);
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

    pub fn is_valid(&self) -> bool {
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

    pub fn ab(&self) -> f64 {
        self.ab
    }
    pub fn bc(&self) -> f64 {
        self.bc
    }
    pub fn ca(&self) -> f64 {
        self.ca
    }
}

impl PartialEq for Angles {
    fn eq(&self, other: &Self) -> bool {
        let self_angles = [self.ab, self.bc, self.ca];
        let mut other_angles = [other.ab, other.bc, other.ca];
        let length = 3;

        for _ in 0..=length {
            if self_angles == other_angles {
                return true;
            }

            other_angles.rotate_left(1);
        }

        other_angles.reverse();

        for _ in 0..=length {
            if self_angles == other_angles {
                return true;
            }

            other_angles.rotate_left(1);
        }

        return false;
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

    #[test]
    fn eq() {
        let angle_base = Angles::new(3.0, 2.0, 4.0);
        let angle_same = Angles::new(3.0, 2.0, 4.0);
        let angle_reverse = Angles::new(4.0, 2.0, 3.0);
        let angle_deviation = Angles::new(4.0, 3.0, 2.0);
        let angle_another = Angles::new(4.0, 3.0, 3.0);

        assert!(angle_base == angle_same);
        assert!(angle_base == angle_reverse);
        assert!(angle_base == angle_deviation);
        assert!(angle_base != angle_another);
    }
}
