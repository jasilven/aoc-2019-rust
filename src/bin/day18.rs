use anyhow::Result;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::io::{BufRead, BufReader};
mod util;

fn parse_input(path: &str) -> Result<HashMap<(isize, isize), char>> {
    let mut result = HashMap::new();
    let file = std::fs::File::open(path)?;

    let (mut x, mut y) = (0, 0);

    for line in BufReader::new(&file).lines() {
        for ch in line?.trim_end().chars() {
            if ch != '#' {
                result.insert((x, y), ch);
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }

    Ok(result)
}


fn bfs(map: &HashMap<(isize, isize), char>) -> Result<usize> {
    let mut points_to_visit: VecDeque<((isize, isize), usize, usize)> = VecDeque::new();
    let mut seen: HashSet<((isize, isize), usize)> = HashSet::new();
    let mut all_keys = 0usize;

    map.values()
        .filter(|ch| ch.is_ascii_lowercase())
        .for_each(|ch| {
            all_keys = all_keys | (1 << (*ch as usize - 'a' as usize));
        });

    let (origin, _) = map
        .iter()
        .find(|(_, val)| *val == &'@')
        .ok_or_else(|| anyhow::anyhow!("origin not found"))?;

    points_to_visit.push_back((*origin, 0, all_keys));
    seen.insert((*origin, all_keys));

    loop {
        let (xy, steps, mut keys) = points_to_visit.pop_front().unwrap();
        let ascii = *map.get(&xy).unwrap() as usize;

        if ascii >= 97 {
            let key = 1 << (ascii - 97);
            if 0 != (keys & key) {
                keys = keys ^ key;
                if keys == 0 {
                    return Ok(steps);
                }
            }
        }


        for neighbour in util::neighbours(&xy) {
            match map.get(&neighbour) {
                None => continue,
                Some(ch) => {
                    if (*ch as usize >= 65) && (*ch as usize <= 90) {
                        let key = 1 << (*ch as usize - 65);
                        if 0 != (keys & key) {
                            continue;
                        }
                    }
                }
            }

            if !seen.contains(&(neighbour, keys)) {
                points_to_visit.push_back((neighbour, steps + 1, keys));
                seen.insert((neighbour, keys));
            }
        }
    }
}

fn solve1(map: &HashMap<(isize, isize), char>) -> Result<usize> {
    bfs(map)
}

fn main() -> Result<()> {
    let map = parse_input("resources/day18-input.txt")?;
    println!("part 1: {}", solve1(&map)?);

    Ok(())
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn part1_test() {
        let tests = vec![
            ("resources/day18-test.txt", 8),
            ("resources/day18-test2.txt", 86),
            ("resources/day18-test3.txt", 132),
            ("resources/day18-test4.txt", 136),
            ("resources/day18-test5.txt", 81),
        ];

        for test in tests {
            let map = parse_input(test.0).unwrap();
            assert_eq!(test.1, solve1(&map).unwrap());
        }
    }
}
