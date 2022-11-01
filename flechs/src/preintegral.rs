use crate::{time::TimeUnit, timeline::Timeline, util::upper_bound};

/// Indicates that this element is integrable.
pub trait Integrable<U>
where
    U: TimeUnit,
{
    type Output: Clone;

    /// Integrates the value.
    fn integrate_within(&self, self_time: U, target_time: U) -> Self::Output;

    /// Accumlates the value.
    fn accumlate(lhs: Self::Output, rhs: Self::Output) -> Self::Output;

    /// Defines zero value of output.
    fn zero() -> Self::Output;
}

#[derive(Debug, Clone)]
pub struct Preintegral<U, V>
where
    U: TimeUnit,
    V: Integrable<U>,
{
    times: Vec<U>,
    items: Vec<V>,
    integrated_values: Vec<V::Output>,
}

impl<U, V> Preintegral<U, V>
where
    U: TimeUnit,
    V: Integrable<U>,
{
    pub fn new(timeline: Timeline<U, V>) -> Preintegral<U, V> {
        let mut pairs = timeline.into_pairs();
        let (first_time, first_value) = pairs.next().expect("invalid timeline");

        let mut times = vec![first_time];
        let mut items = vec![first_value];
        let mut integrated_values = vec![V::zero()];
        for (time, value) in pairs {
            let last_time = times.last().expect("must have item");
            let last_value = items.last().expect("must have item");
            let last_integral = integrated_values.last().expect("must have item");
            let section = last_value.integrate_within(*last_time, time);
            let integrated_value = V::accumlate(last_integral.clone(), section);
            times.push(time);
            items.push(value);
            integrated_values.push(integrated_value);
        }

        Preintegral {
            times,
            integrated_values,
            items,
        }
    }

    pub fn fetch(&self, time: U) -> V::Output {
        let base = upper_bound(&self.times, &time) - 1;
        let section = self.items[base].integrate_within(self.times[base], time);
        let sum = V::accumlate(self.integrated_values[base].clone(), section);

        sum
    }
}
