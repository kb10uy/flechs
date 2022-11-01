use crate::{
    time::TimeUnit,
    util::{lower_bound, upper_bound},
};

use std::iter::zip;

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
}
