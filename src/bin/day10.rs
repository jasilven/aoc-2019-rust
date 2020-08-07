use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
mod float64;
use float64::F64;

fn parse_input(path: &str) -> Result<HashSet<(usize, usize)>> {
    let mut result = HashSet::new();

    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut y = 0;
    for line in reader.lines() {
        for (x, ch) in line?.chars().enumerate() {
            if ch == '#' {
                result.insert((x, y));
            }
        }
        y += 1;
    }

    Ok(result)
}

fn get_connections(
    map: &HashSet<(usize, usize)>,
    a: &(usize, usize),
) -> HashMap<F64, Vec<(usize, usize)>> {
    let mut result: HashMap<F64, Vec<(usize, usize)>> = HashMap::new();

    for b in map.iter() {
        if a != b {
            let ab = (b.0 as f64 - a.0 as f64, b.1 as f64 - a.1 as f64);
            let degrees = ab.1.atan2(ab.0) / (std::f64::consts::PI / 180.0) + 180.0; // 0..360
            let ab_degrees = F64::new(degrees);

            if result.contains_key(&ab_degrees) {
                let entry = result.get_mut(&ab_degrees).unwrap();
                entry.push(*b);
            } else {
                result.insert(ab_degrees, vec![*b]);
            }
        }
    }
    for entry in result.iter_mut() {
        entry.1.sort_by_key(|item| item.1);
    }

    result
}

fn solve1(map: &HashSet<(usize, usize)>) -> Result<((usize, usize), usize)> {
    let mut result = ((0, 0), 0);

    for a in map.iter() {
        let degrees_points = get_connections(&map, a);
        let len = degrees_points.len();
        if len > result.1 {
            result = (*a, len);
        }
    }

    Ok(result)
}

fn solve2(map: &HashSet<(usize, usize)>, point: &(usize, usize), n: usize) -> usize {
    let degrees_points = get_connections(&map, point);

    let mut vec = vec![];
    for (degree, val) in degrees_points {
        let mut f = degree.get();
        if f < 90.0 {
            f = 270.0 + f;
        } else {
            f = f - 90.0;
        }
        vec.push((F64::new(f + 90.0), val));
    }

    vec.sort_by(|a, b| a.0.get().partial_cmp(&b.0.get()).unwrap());

    let mut all_asteroids: Vec<(f64, (usize, usize))> = vec![];

    for (degree, asteroids) in vec.iter() {
        for (i, asteroid) in asteroids.iter().enumerate() {
            all_asteroids.push((degree.get() + (i * 360) as f64, *asteroid))
        }
    }
    all_asteroids.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    (all_asteroids[n].1).0 * 100 + (all_asteroids[n].1).1
}

fn main() -> Result<()> {
    let map = parse_input("resources/day10-input.txt")?;

    let part1 = solve1(&map)?;
    println!("part 1: {:?}", part1.0);

    let map = parse_input("resources/day10-input.txt")?;
    println!("part 2: {:?}", solve2(&map, &part1.0, 199));

    Ok(())
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn solve1_test() {
        let tests = vec![
            ("resources/day10-test.txt", 8),
            ("resources/day10-test2.txt", 33),
            ("resources/day10-test3.txt", 35),
            ("resources/day10-test4.txt", 41),
            ("resources/day10-test5.txt", 210),
        ];
        for test in tests {
            let map = parse_input(test.0).unwrap();
            assert_eq!(test.1, solve1(&map).unwrap().1);
        }
    }
    #[test]
    fn solve2_test() {
        let map = parse_input("resources/day10-test6.txt").unwrap();
        assert_eq!(15 * 100 + 1, solve2(&map, &(8, 3), 8));
    }
}
