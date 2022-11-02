use crate::{
    time::TimeUnit,
    util::{lower_bound, upper_bound},
};

use std::{cmp::Ordering, iter::zip};

use thiserror::Error as ThisError;

#[derive(Debug, Clone, PartialEq, Eq, ThisError)]
pub enum TimelineError {
    /// Timelines with duplicate times are to merge.
    #[error("merging timelines that have duplicate times")]
    HasDuplicateTimes,
}

/// Represents a item timeline.
#[derive(Debug, Clone)]
pub struct Timeline<U, V> {
    times: Vec<U>,
    items: Vec<V>,
}

impl<U, V> Timeline<U, V>
where
    U: TimeUnit,
{
    /// Creates empty timeline.
    pub fn new() -> Timeline<U, V> {
        Timeline {
            times: vec![],
            items: vec![],
        }
    }

    /// Returns iterator of times.
    pub fn times(&self) -> impl Iterator<Item = U> + '_ {
        self.times.iter().copied()
    }

    /// Returns iterator of items.
    pub fn items(&self) -> impl Iterator<Item = &V> {
        self.items.iter()
    }

    /// Consumes itself and returns pair iterator.
    pub fn into_pairs(self) -> impl Iterator<Item = (U, V)> {
        zip(self.times, self.items)
    }

    /// Appends a new item pair.
    /// if any pair exists, new one must be later than last item, or will panic.
    pub fn append(&mut self, time: U, item: V) {
        assert!(self.times.len() == self.items.len());

        if let Some(last_time) = self.times.last() {
            assert!(&time > last_time, "invalid time order");
        }
        self.times.push(time);
        self.items.push(item);
    }

    /// Inserts pair in correct position.
    pub fn insert(&mut self, time: U, item: V) {
        assert!(self.times.len() == self.items.len());

        let target_index = upper_bound(&self.times, &time);
        self.times.insert(target_index, time);
        self.items.insert(target_index, item);
    }

    /// Gets latest item.
    pub fn latest_item(&self, time: U) -> Option<&V> {
        let left = upper_bound(&self.times, &time);
        if left > 0 {
            Some(&self.items[left - 1])
        } else {
            None
        }
    }

    /// Gets latest time items slice.
    pub fn latest_slice(&self, time: U) -> &[V] {
        let left = lower_bound(&self.times, &time);
        if let Some(latest_time) = self.times.get(left) {
            let right = upper_bound(&self.times, latest_time);
            &self.items[left..right]
        } else {
            &self.items[left..left]
        }
    }

    /// Returns whether this timeline has duplicate time steps.
    pub fn has_duplicate_times(&self) -> bool {
        let mut times = self.times();
        let mut last_time = match times.next() {
            Some(t) => t,
            None => return false,
        };
        // MEMO: try_fold() may be used
        for time in times {
            if time == last_time {
                return true;
            }
            last_time = time;
        }
        false
    }

    /// Merges two timeline into one timeline of tuples.
    pub fn merge<W>(
        self,
        right: Timeline<U, W>,
    ) -> Result<Timeline<U, (Option<V>, Option<W>)>, TimelineError> {
        if self.has_duplicate_times() || right.has_duplicate_times() {
            return Err(TimelineError::HasDuplicateTimes);
        }
        let mut left_pairs = self.into_pairs();
        let mut right_pairs = right.into_pairs();
        let mut last_left = left_pairs.next();
        let mut last_right = right_pairs.next();

        let mut tl = Timeline::new();
        loop {
            match (last_left, last_right) {
                (Some((lt, li)), Some((rt, ri))) => {
                    match lt.partial_cmp(&rt).expect("not supported") {
                        Ordering::Less => {
                            tl.append(lt, (Some(li), None));
                            last_left = left_pairs.next();
                            last_right = Some((rt, ri));
                        }
                        Ordering::Equal => {
                            tl.append(lt, (Some(li), Some(ri)));
                            last_left = left_pairs.next();
                            last_right = right_pairs.next();
                        }
                        Ordering::Greater => {
                            tl.append(rt, (None, Some(ri)));
                            last_left = Some((lt, li));
                            last_right = right_pairs.next();
                        }
                    }
                }
                (Some((lt, li)), None) => {
                    tl.append(lt, (Some(li), None));
                    last_left = left_pairs.next();
                    last_right = None;
                }
                (None, Some((rt, ri))) => {
                    tl.append(rt, (None, Some(ri)));
                    last_left = None;
                    last_right = right_pairs.next();
                }
                (None, None) => break,
            }
        }
        Ok(tl)
    }
}

impl<U, V> FromIterator<(U, V)> for Timeline<U, V>
where
    U: TimeUnit,
{
    fn from_iter<T: IntoIterator<Item = (U, V)>>(iter: T) -> Self {
        let mut tl = Timeline::new();
        for (time, item) in iter {
            tl.append(time, item);
        }
        tl
    }
}

#[macro_export]
macro_rules! timeline {
    { $( [ $m:literal : $sn:literal / $sd:literal ] : $v:expr , )* } => (
        {
            let mut tl = $crate::timeline::Timeline::new();
            $(
                tl.append($crate::time::Instant::new_parts($m, $sn, $sd), $v);
            )*
            tl
        }
    );
    { $( [ $t:expr ] : $v:expr , )* } => (
        {
            let mut tl = $crate::timeline::Timeline::new();
            $(
                tl.append($t, $v);
            )*
            tl
        }
    );
}

#[cfg(test)]
mod tests {
    use crate::instant;

    #[test]
    fn basic_timeline_works() {
        let tl = timeline! {
            [0:0/1]: 7,
            [7:0/1]: 4,
        };

        assert_eq!(
            tl.latest_item(instant![0:0/1]),
            Some(&7),
            "timeline fetches correct item"
        );
        assert_eq!(
            tl.latest_item(instant![2:3/4]),
            Some(&7),
            "timeline fetches correct item"
        );
        assert_eq!(
            tl.latest_item(instant![7:0/1]),
            Some(&4),
            "timeline fetches correct item"
        );
        assert_eq!(
            tl.latest_item(instant![100:0/1]),
            Some(&4),
            "timeline fetches correct item"
        );
    }

    #[test]
    fn advanced_timeline_works() {
        // TODO: write test
    }
}
