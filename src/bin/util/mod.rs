fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    use std::cmp::{max, min};
    max(a.0, b.0) - min(a.0, b.0) + max(a.1, b.1) - min(a.1, b.1)
}

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn manhattan_distance_test() {
        assert_eq!(0, manhattan_distance(&(1, 1), &(1, 1)));
        assert_eq!(8, manhattan_distance(&(0, 0), &(4, 4)));
    }
}
