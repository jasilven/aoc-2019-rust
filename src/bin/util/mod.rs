pub fn manhattan_distance(a: &(isize, isize), b: &(isize, isize)) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn neighbours(pos: &(isize, isize)) -> Vec<(isize, isize)> {
    [
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
    ]
    .to_vec()
}

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn manhattan_distance_test() {
        assert_eq!(0, manhattan_distance(&(1, 1), &(1, 1)));
        assert_eq!(8, manhattan_distance(&(0, 0), &(4, 4)));
        assert_eq!(2, manhattan_distance(&(-1, 0), &(1, 0)));
    }
}
