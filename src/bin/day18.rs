use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, BufReader};
mod util;

fn parse_input(path: &str) -> Result<HashMap<(isize, isize), char>> {
    let mut result = HashMap::new();
    let file = std::fs::File::open(path)?;

    for (y, line) in BufReader::new(&file).lines().enumerate() {
        for (x, ch) in line?.trim_end().chars().enumerate() {
            if ch != '#' {
                result.insert((x as isize, y as isize), ch);
            }
        }
    }

    Ok(result)
}

fn bfs(map: &HashMap<(isize, isize), char>, origin: &(isize, isize)) -> Result<usize> {
    let mut points_to_visit: VecDeque<((isize, isize), usize, usize)> = VecDeque::new();
    let mut seen: HashSet<((isize, isize), usize)> = HashSet::new();
    let mut all_keys = 0usize;

    map.values()
        .filter(|ch| ch.is_ascii_lowercase())
        .for_each(|ch| {
            all_keys = all_keys | (1 << (*ch as usize - 'a' as usize));
        });

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

fn solve1(map: &HashMap<(isize, isize), char>, origin: &(isize, isize)) -> Result<usize> {
    bfs(map, origin)
}

fn partition_map(
    map: &mut HashMap<(isize, isize), char>,
    origin: &(isize, isize),
) -> Vec<HashMap<(isize, isize), char>> {
    let origin_deltas = [(-1, -1), (1, -1), (1, 1), (-1, 1)];
    let del_deltas = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    map.remove(origin);

    for (dx, dy) in del_deltas.iter() {
        map.remove(&(origin.0 + dx, origin.1 + dy));
    }
    for (dx, dy) in origin_deltas.iter() {
        map.insert((origin.0 + dx, origin.1 + dy), '@');
    }
    let mut result = vec![
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];

    map.iter().for_each(|(xy, ch)| {
        if xy.0 < origin.0 && xy.1 < origin.1 {
            result[0].insert(*xy, *ch);
        } else if xy.0 > origin.0 && xy.1 < origin.1 {
            result[1].insert(*xy, *ch);
        } else if xy.0 < origin.0 && xy.1 > origin.1 {
            result[2].insert(*xy, *ch);
        } else if xy.0 > origin.0 && xy.1 > origin.1 {
            result[3].insert(*xy, *ch);
        }
    });

    result
}

fn open_doors(map: &mut HashMap<(isize, isize), char>) {
    let keys: HashSet<char> = map
        .iter()
        .filter(|(_, ch)| ch.is_ascii_lowercase())
        .map(|(_, ch)| *ch)
        .collect();

    for (_, ch) in map.iter_mut().filter(|(_, ch)| ch.is_ascii_uppercase()) {
        if !keys.contains(&ch.to_ascii_lowercase()) {
            *ch = '.';
        }
    }
}

fn solve2(map: &mut HashMap<(isize, isize), char>, origin: &(isize, isize)) -> Result<usize> {
    let mut result = 0;

    let maps = partition_map(map, origin);

    for mut m in maps {
        open_doors(&mut m);

        let (origin, _) = m
            .iter()
            .find(|(_, val)| *val == &'@')
            .ok_or_else(|| anyhow::anyhow!("origin not found!"))?;
        result += bfs(&m, origin)?;
    }

    Ok(result)
}

fn main() -> Result<()> {
    let map = parse_input("resources/day18-input.txt")?;
    let mut map2 = map.clone();

    let (origin, _) = map
        .iter()
        .find(|(_, val)| *val == &'@')
        .ok_or_else(|| anyhow::anyhow!("origin not found"))?;

    println!("part 1: {}", solve1(&map, &origin)?);
    println!("part 2: {}", solve2(&mut map2, &origin)?);

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
            let (origin, _) = map.iter().find(|(_, val)| *val == &'@').unwrap();
            assert_eq!(test.1, solve1(&map, &origin).unwrap());
        }
    }
}
