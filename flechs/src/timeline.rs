/// Represents a item timeline.
pub struct Timeline<U, V> {
    times: Vec<U>,
    items: Vec<V>,
}

impl<U, V> Timeline<U, V>
where
    U: PartialOrd,
{
    /// Creates empty timeline.
    pub fn new() -> Timeline<U, V> {
        Timeline {
            times: vec![],
            items: vec![],
        }
    }

    /// Returns iterator of times.
    pub fn times(&self) -> impl Iterator<Item = &U> {
        self.times.iter()
    }

    /// Returns iterator of items.
    pub fn items(&self) -> impl Iterator<Item = &V> {
        self.items.iter()
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
        let left = upper_bound(&self.times, time);
        if left > 0 {
            Some(&self.items[left - 1])
        } else {
            None
        }
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
pub fn lower_bound<T: PartialOrd>(target: &[T], item: &T) -> usize {
    let mut search_range = 0..(target.len());
    while search_range.len() > 0 {
        let mid = search_range.len() / 2 + search_range.start;
        search_range = if item <= &target[mid] {
            (search_range.start)..mid
        } else {
            (mid + 1)..(search_range.end)
        };
    }

    search_range.start
}

/// Searches upper bound index for specified time.
pub fn upper_bound<T: PartialOrd>(target: &[T], item: &T) -> usize {
    let mut search_range = 0..(target.len());
    while search_range.len() > 0 {
        let mid = search_range.len() / 2 + search_range.start;
        search_range = if item < &target[mid] {
            (search_range.start)..mid
        } else {
            (mid + 1)..(search_range.end)
        };
    }
    search_range.start
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
}

#[cfg(test)]
mod tests {
    use super::{lower_bound, upper_bound};
    use crate::instant;

    #[test]
    fn bound_functions_work() {
        let source = vec![1, 2, 4, 8, 16, 32, 64, 128];
        assert_eq!(lower_bound(&source, &1), 0);
        assert_eq!(lower_bound(&source, &2), 1);
        assert_eq!(lower_bound(&source, &3), 2);
        assert_eq!(lower_bound(&source, &4), 2);
        assert_eq!(lower_bound(&source, &20), 5);
        assert_eq!(lower_bound(&source, &256), 8);

        let source = vec![1, 1, 1, 1, 16, 16, 16, 16, 256];
        assert_eq!(lower_bound(&source, &0), 0);
        assert_eq!(lower_bound(&source, &1), 0);
        assert_eq!(lower_bound(&source, &2), 4);
        assert_eq!(lower_bound(&source, &15), 4);
        assert_eq!(lower_bound(&source, &16), 4);
        assert_eq!(lower_bound(&source, &17), 8);
        assert_eq!(lower_bound(&source, &512), 9);

        let source = vec![1, 2, 4, 8, 16, 32, 64, 128];
        assert_eq!(upper_bound(&source, &1), 1);
        assert_eq!(upper_bound(&source, &2), 2);
        assert_eq!(upper_bound(&source, &3), 2);
        assert_eq!(upper_bound(&source, &4), 3);
        assert_eq!(upper_bound(&source, &20), 5);
        assert_eq!(upper_bound(&source, &256), 8);

        let source = vec![1, 1, 1, 1, 16, 16, 16, 16, 256];
        assert_eq!(upper_bound(&source, &0), 0);
        assert_eq!(upper_bound(&source, &1), 4);
        assert_eq!(upper_bound(&source, &2), 4);
        assert_eq!(upper_bound(&source, &15), 4);
        assert_eq!(upper_bound(&source, &16), 8);
        assert_eq!(upper_bound(&source, &17), 8);
        assert_eq!(upper_bound(&source, &512), 9);
    }

    #[test]
    fn basic_timeline_works() {
        let tl = timeline! {
            [0:0/1]: 7,
            [7:0/1]: 4,
        };

        assert_eq!(
            tl.latest_item(&instant![0:0/1]),
            Some(&7),
            "timeline fetches correct item"
        );
        assert_eq!(
            tl.latest_item(&instant![2:3/4]),
            Some(&7),
            "timeline fetches correct item"
        );
        assert_eq!(
            tl.latest_item(&instant![7:0/1]),
            Some(&4),
            "timeline fetches correct item"
        );
        assert_eq!(
            tl.latest_item(&instant![100:0/1]),
            Some(&4),
            "timeline fetches correct item"
        );
    }
}
