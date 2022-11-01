//! Contains various value types.

use num::{rational::Ratio, Zero};

use crate::preintegral::Integrable;

/// Represents beat event.
pub struct Beat(pub Ratio<usize>);

impl Integrable<usize> for Beat {
    type Output = Ratio<usize>;

    fn integrate_within(&self, self_time: usize, target_time: usize) -> Self::Output {
        self.0 * (target_time - self_time) as usize
    }

    fn accumlate(lhs: Self::Output, rhs: Self::Output) -> Self::Output {
        lhs + rhs
    }

    fn zero() -> Self::Output {
        Ratio::zero()
    }
}
