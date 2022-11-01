//! Basic time structs.

use num::{rational::Ratio, Integer};
use thiserror::Error as ThisError;

/// Indicates that this type can be used as time unit in flechs.
pub trait TimeUnit: PartialOrd + Copy {}

impl<N> TimeUnit for N where N: Copy + Integer {}

/// Represents a paticular instant time in chart/score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    /// 0-based measure number.
    measure: usize,

    /// time in measure.
    submeasure: Ratio<usize>,
}

impl TimeUnit for Instant {}

/// Represents an error about `Instant`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ThisError)]
pub enum InstantError {
    /// over-1 rational specified for submeasure.
    #[error("too big submeasure rational: {0}")]
    OverSubmeasure(Ratio<usize>),
}

impl Instant {
    /// Creates new instant.
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

    /// Creates new instant with parts.
    pub const fn new_parts(measure: usize, sub_numer: usize, sub_denom: usize) -> Instant {
        if sub_denom == 0 || sub_numer >= sub_denom {
            panic!("invalid submeasure");
        } else {
            Instant {
                measure,
                submeasure: Ratio::new_raw(sub_numer, sub_denom),
            }
        }
    }

    pub const fn measure(&self) -> usize {
        self.measure
    }

    pub const fn submeasure(&self) -> Ratio<usize> {
        self.submeasure
    }
}

/// Constructs an `Instant` in const context.
#[macro_export]
macro_rules! instant {
    [$m:literal : $sn:literal / $sd:literal] => {
        $crate::time::Instant::new_parts($m, $sn, $sd)
    };
}

#[cfg(test)]
mod tests {
    use super::{Instant, InstantError};

    use num::rational::Ratio;

    #[test]
    fn instant_validation_works() {
        assert!(
            Instant::new(0, Ratio::new(0, 1)).is_ok(),
            "Valid Instant pass"
        );
        assert!(
            Instant::new(2, Ratio::new(1, 4)).is_ok(),
            "Valid Instant pass"
        );
        assert!(
            Instant::new(2, Ratio::new(15, 16)).is_ok(),
            "Valid Instant pass"
        );

        assert_eq!(
            Instant::new(0, Ratio::new(4, 4)),
            Err(InstantError::OverSubmeasure(Ratio::new(4, 4))),
            "Invalid instant passes"
        );
        assert_eq!(
            Instant::new(0, Ratio::new(8, 4)),
            Err(InstantError::OverSubmeasure(Ratio::new(8, 4))),
            "Invalid instant passes"
        );
    }

    #[test]
    fn instant_macro_works() {
        assert_eq!(instant![0:0/1].measure, 0, "Instant macro works");
        assert_eq!(instant![1:3/4].measure, 1, "Instant macro works");
    }

    #[test]
    #[should_panic(expected = "invalid submeasure")]
    fn invalid_instant_macro_panicks() {
        instant![0:1/1];
    }
}
