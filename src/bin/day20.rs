use anyhow::{anyhow, Result};
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::FromStr;
use std::{collections::HashMap, fmt::Display};

type Xyz = (usize, usize, usize);

#[derive(Debug)]
struct Port {
    a: (usize, usize),
    b: (usize, usize),
    id: String,
}

#[derive(Debug)]
enum Tile {
    Open,
    InnerPortal(String),
    OuterPortal(String),
}

#[derive(Eq, PartialEq, Debug)]
struct Point(Xyz, usize);

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Maze {
    map: HashMap<(usize, usize), Tile>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl Maze {
    fn neighbours(&self, xyz: &Xyz) -> HashSet<Xyz> {
        let nbours = [
            (xyz.0 + 1, xyz.1),
            (xyz.0 - 1, xyz.1),
            (xyz.0, xyz.1 + 1),
            (xyz.0, xyz.1 - 1),
        ];
        let mut result: HashSet<Xyz> = nbours
            .iter()
            .filter(|p| self.map.contains_key(*p))
            .map(|(x, y)| (*x, *y, xyz.2))
            .collect();
        match self.map.get(&(xyz.0, xyz.1)) {
            Some(Tile::InnerPortal(s)) | Some(Tile::OuterPortal(s)) => {
                for (xyz2, tile) in self.map.iter() {
                    let xyz2 = (xyz2.0, xyz2.1, xyz.2);
                    match tile {
                        Tile::InnerPortal(ss) | Tile::OuterPortal(ss)
                            if ((ss.as_str() == s.as_str()) && (xyz != &xyz2)) =>
                        {
                            result.insert(xyz2);
                            break;
                        }
                        _ => {}
                    };
                }
            }
            _ => {}
        }
        result
    }

    fn neighbours2(&self, xyz: &Xyz) -> HashSet<Xyz> {
        let nbours = [
            (xyz.0 + 1, xyz.1),
            (xyz.0 - 1, xyz.1),
            (xyz.0, xyz.1 + 1),
            (xyz.0, xyz.1 - 1),
        ];
        let mut result: HashSet<Xyz> = nbours
            .iter()
            .filter(|p| self.map.contains_key(*p))
            .map(|(x, y)| (*x, *y, xyz.2))
            .collect();
        match self.map.get(&(xyz.0, xyz.1)) {
            Some(Tile::InnerPortal(s)) => {
                for (xyz2, tile) in self.map.iter() {
                    match tile {
                        Tile::OuterPortal(ss)
                            if ((ss.as_str() == s.as_str()) && (&(xyz.0, xyz.1) != xyz2)) =>
                        {
                            result.insert((xyz2.0, xyz2.1, xyz.2 + 1));
                            break;
                        }
                        _ => {}
                    };
                }
            }
            Some(Tile::OuterPortal(s)) if xyz.2 > 0 => {
                for (xyz2, tile) in self.map.iter() {
                    match tile {
                        Tile::InnerPortal(ss)
                            if ((ss.as_str() == s.as_str()) && (&(xyz.0, xyz.1) != xyz2)) =>
                        {
                            result.insert((xyz2.0, xyz2.1, xyz.2 - 1));
                            break;
                        }
                        _ => {}
                    };
                }
            }
            _ => {}
        }
        result
    }

    fn port(&self, port: &str) -> Option<(usize, usize, usize)> {
        self.map
            .iter()
            .find(|(_, tile)| match tile {
                Tile::InnerPortal(aa) if aa.as_str() == port => true,
                Tile::OuterPortal(aa) if aa.as_str() == port => true,
                _ => false,
            })
            .map(|(xy, _)| (xy.0, xy.1, 0))
    }
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let mut ports = vec![];
        let mut letters: HashMap<(usize, usize), char> = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '.' {
                    map.insert((x, y), Tile::Open);
                } else if ch.is_alphabetic() {
                    if let Some((xy, ch2)) = letters
                        .iter()
                        .filter(|(&xy, _)| distance_xy(&(x, y), &xy) == 1)
                        .next()
                    {
                        ports.push(Port {
                            a: *xy,
                            b: (x, y),
                            id: match distance_xy(&(0, 0), &(x, y)) < distance_xy(&(0, 0), xy) {
                                true => format!("{}{}", ch, ch2),
                                _ => format!("{}{}", ch2, ch),
                            },
                        });
                    } else {
                        letters.insert((x, y), ch);
                    }
                }
            }
        }

        let min_x = *map.iter().map(|((x, _), _)| x).min().unwrap();
        let max_x = *map.iter().map(|((x, _), _)| x).max().unwrap();
        let min_y = *map.iter().map(|((_, y), _)| y).min().unwrap();
        let max_y = *map.iter().map(|((_, y), _)| y).max().unwrap();

        // locate and update portals to the map
        for port in ports {
            let xy = map
                .iter_mut()
                .find(|(xy, _)| distance_xy(&port.a, *xy) == 1 || distance_xy(&port.b, *xy) == 1)
                .unwrap();
            if (xy.0 .0 == min_x) || (xy.0 .0 == max_x) || (xy.0 .1 == min_y) || (xy.0 .1 == max_y)
            {
                *xy.1 = Tile::OuterPortal(port.id);
            } else {
                *xy.1 = Tile::InnerPortal(port.id);
            }
        }

        Ok(Self {
            map,
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                match self.map.get(&(x, y)) {
                    Some(Tile::Open) => {
                        f.write_str(".")?;
                    }
                    None => {
                        f.write_str(" ")?;
                    }
                    Some(Tile::InnerPortal(_)) | Some(Tile::OuterPortal(_)) => {
                        f.write_str("+")?;
                    }
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn distance_xy(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let dcol = a.0.max(b.0) - a.0.min(b.0);
    let drow = a.1.max(b.1) - a.1.min(b.1);
    drow.max(dcol) + drow.min(dcol)
}

fn solve1(maze: &Maze) -> Result<usize> {
    let start = maze
        .port("AA")
        .ok_or_else(|| anyhow!("'AA'-portal not found"))?;
    let end = maze
        .port("ZZ")
        .ok_or_else(|| anyhow!("'ZZ'-portal not found"))?;
    let mut seen: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut unseen: BinaryHeap<Point> = BinaryHeap::new();
    unseen.push(Point(start, 0));

    let result = loop {
        let current = unseen.pop().unwrap();

        if current.0 == end {
            break current.1;
        }
        seen.insert(current.0);
        let neighbours: Vec<(usize, usize, usize)> = maze
            .neighbours(&current.0)
            .difference(&seen)
            .cloned()
            .collect();
        unseen.extend(neighbours.iter().map(|xy| Point(*xy, 1 + current.1)));
    };

    Ok(result)
}

fn solve2(maze: &Maze) -> Result<usize> {
    let start = maze
        .port("AA")
        .ok_or_else(|| anyhow!("'AA'-portal not found"))?;
    let end = maze
        .port("ZZ")
        .ok_or_else(|| anyhow!("'ZZ'-portal not found"))?;
    let mut seen: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut unseen: BinaryHeap<Point> = BinaryHeap::new();
    unseen.push(Point(start, 0));

    let result = loop {
        let current = unseen.pop().unwrap();

        if current.0 == end {
            break current.1;
        }
        seen.insert(current.0);
        let neighbours: Vec<(usize, usize, usize)> = maze
            .neighbours2(&current.0)
            .difference(&seen)
            .cloned()
            .collect();
        unseen.extend(neighbours.iter().map(|xyz| Point(*xyz, 1 + current.1)));
    };

    Ok(result)
}
fn main() -> Result<()> {
    let maze = Maze::from_str(&read_to_string("resources/day20-input.txt")?)?;
    let part1 = solve1(&maze).unwrap();
    println!("part 1: {}", &part1);
    let part2 = solve2(&maze).unwrap();
    println!("part 2: {}", &part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(0, distance_xy(&(1, 1), &(1, 1)));
        assert_eq!(1, distance_xy(&(0, 1), &(1, 1)));
        assert_eq!(1, distance_xy(&(1, 1), &(1, 0)));
        assert_eq!(2, distance_xy(&(1, 1), &(0, 0)));
        assert_eq!(10, distance_xy(&(5, 5), &(0, 0)));
        assert_eq!(10, distance_xy(&(0, 0), &(5, 5)));
    }

    #[test]
    fn test_neighbours() {
        let maze = Maze::from_str(&read_to_string("resources/day20-test.txt").unwrap()).unwrap();
        // case 1: no portal
        let t1 = maze.neighbours(&(9, 2, 0));
        assert_eq!(t1.len(), 1);

        // case 2: with portal
        let t1 = maze.neighbours(&(9, 6, 0));
        assert_eq!(t1.len(), 2);
    }

    #[test]
    fn test_part1_case1() {
        let maze = Maze::from_str(&read_to_string("resources/day20-test.txt").unwrap()).unwrap();
        let result = solve1(&maze).unwrap();
        assert_eq!(23, result);
    }

    #[test]
    fn test_part1_case2() {
        let maze = Maze::from_str(&read_to_string("resources/day20-test2.txt").unwrap()).unwrap();
        let result = solve1(&maze).unwrap();
        assert_eq!(58, result);
    }

    #[test]
    fn test_part2() {
        let maze = Maze::from_str(&read_to_string("resources/day20-test.txt").unwrap()).unwrap();
        let result = solve2(&maze).unwrap();
        assert_eq!(26, result);
    }

    #[test]
    fn test_part2_case3() {
        let maze = Maze::from_str(&read_to_string("resources/day20-test3.txt").unwrap()).unwrap();
        let result = solve2(&maze).unwrap();
        assert_eq!(396, result);
    }
}
