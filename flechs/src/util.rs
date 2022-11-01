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

#[cfg(test)]
mod tests {
    use super::{lower_bound, upper_bound};

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
}
