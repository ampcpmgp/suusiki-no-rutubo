const TOTAL_ANGLE: f64 = 180.0;

pub struct Triangle {}

pub struct Sides {
    a: f64,
    b: f64,
    c: f64,
}

pub struct Angles {
    ab: f64,
    bc: f64,
    ca: f64,
}

pub enum AngleError {
    LessThanZero(Angles::ab),
    OverTotalAngle(&'static str, f64),
}

impl Angles {
    ///  ```
    /// use suusiki_no_rutubo::geometry::triangle::{AngleError, Angles};
    ///
    /// match Angles::new(10.0, 20.0) {
    ///     Ok(angle) => assert_eq!(angle.ca(), 150.0),
    ///
    ///     Err(AngleError::LessThanZero(name, angle)) => {
    ///         panic!("{} is {}, less than zero.", name, angle)
    ///     }
    ///
    ///     Err(AngleError::OverTotalAngle(name, angle)) => {
    ///         panic!("{} is {}, over total angle.", name, angle)
    ///     }
    /// }
    pub fn new(ab: f64, bc: f64, ca: f64) -> Angles {
        Angles { ab, bc, ca }
    }

    pub fn left_angle() {}

    ///
    /// aaa
    ///
    pub fn new_a(ab: f64, bc: f64) -> Result<Angles, AngleError> {
        Angles::check_valid(ab, "ab")?;
        Angles::check_valid(bc, "bc")?;

        let ca: f64 = TOTAL_ANGLE - ab - bc;
        Angles::check_valid(ca, "ca")?;

        Ok(Angles { ab, bc, ca })
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

    fn check_valid(angle: f64, name: &'static str) -> Result<(), AngleError> {
        match angle {
            angle if angle <= 0.0 => Err(AngleError::LessThanZero(name, angle)),
            angle if angle >= TOTAL_ANGLE => Err(AngleError::OverTotalAngle(name, angle)),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_test() {
        match Angles::new(10.0, 20.0) {
            Ok(angle) => assert_eq!(angle.ca(), 150.0),

            Err(AngleError::LessThanZero(name, angle)) => {
                panic!("{} is {}, less than zero.", name, angle)
            }

            Err(AngleError::OverTotalAngle(name, angle)) => {
                panic!("{} is {}, over total angle.", name, angle)
            }
        }
    }
}
