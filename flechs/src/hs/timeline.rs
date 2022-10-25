use std::cmp::Ordering;

/// Can be used as time unit for `Timeline`.
pub trait TimeUnit: Ord {
    fn zero() -> Self;
}

/// Represents a item timeline.
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
    pub fn latest_item(&self, time: &U) -> Option<&V> {
        let left = lower_bound(&self.times, time);
        self.items.get(left)
    }

    /// Gets latest time items slice.
    pub fn latest_slice(&self, time: &U) -> &[V] {
        let left = lower_bound(&self.times, time);
        if let Some(latest_time) = self.times.get(left) {
            let right = upper_bound(&self.times, latest_time);
            &self.items[left..right]
        } else {
            &self.items[left..left]
        }
    }
}

/// Searches lower bound index for specified time.
fn lower_bound<T: Ord>(target: &[T], item: &T) -> usize {
    let mut search_range = 0..(target.len());
    while search_range.len() > 0 {
        let mid = search_range.len() / 2 + search_range.start;
        search_range = match item.cmp(&target[mid]) {
            Ordering::Less | Ordering::Equal => (search_range.start)..mid,
            Ordering::Greater => mid..(search_range.end),
        };
    }
    search_range.start
}

/// Searches upper bound index for specified time.
fn upper_bound<T: Ord>(target: &[T], item: &T) -> usize {
    let mut search_range = 0..(target.len());
    while search_range.len() > 0 {
        let mid = search_range.len() / 2 + search_range.start;
        search_range = match item.cmp(&target[mid]) {
            Ordering::Less => (search_range.start)..mid,
            Ordering::Greater | Ordering::Equal => mid..(search_range.end),
        };
    }
    search_range.start
}
