//! Basic time structs.

use num::rational::Ratio;
use thiserror::Error as ThisError;

/// Represents a paticular instant time in chart/score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    /// 0-based measure number.
    measure: usize,

    /// time in measure.
    submeasure: Ratio<usize>,
}

/// Represents an error about `Instant`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ThisError)]
pub enum InstantError {
    /// over-1 rational specified for submeasure.
    #[error("too big submeasure rational: {0}")]
    OverSubmeasure(Ratio<usize>),
}

impl Instant {
    pub fn new(measure: usize, submeasure: Ratio<usize>) -> Result<Instant, InstantError> {
        if submeasure >= Ratio::new(1, 1) {
            Err(InstantError::OverSubmeasure(submeasure))
        } else {
            Ok(Instant {
                measure,
                submeasure,
            })
        }
    }

    pub const fn measure(&self) -> usize {
        self.measure
    }

    pub const fn submeasure(&self) -> Ratio<usize> {
        self.submeasure
    }
}

#[cfg(test)]
mod tests {
    use super::{Instant, InstantError};

    use num::rational::Ratio;

    #[test]
    fn instant_validation_works() {
        assert!(
            Instant::new(0, Ratio::new(0, 0)).is_ok(),
            "Valid Instant does not pass"
        );
        assert!(
            Instant::new(1, Ratio::new(0, 0)).is_ok(),
            "Valid Instant does not pass"
        );
        assert!(
            Instant::new(2, Ratio::new(1, 4)).is_ok(),
            "Valid Instant does not pass"
        );
        assert!(
            Instant::new(2, Ratio::new(3, 4)).is_ok(),
            "Valid Instant does not pass"
        );
        assert!(
            Instant::new(2, Ratio::new(15, 16)).is_ok(),
            "Valid Instant does not pass"
        );

        assert_eq!(
            Instant::new(0, Ratio::new(4, 4)),
            Err(InstantError::OverSubmeasure(Ratio::new(4, 4))),
            "Invalid instant passes"
        );
        assert_eq!(
            Instant::new(0, Ratio::new(8, 4)),
            Err(InstantError::OverSubmeasure(Ratio::new(4, 4))),
            "Invalid instant passes"
        );
    }
}
