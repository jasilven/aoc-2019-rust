use anyhow::{anyhow, Result};
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::FromStr;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
struct Port {
    a: (usize, usize),
    b: (usize, usize),
    id: String,
}

#[derive(Debug)]
enum Tile {
    Open,
    Portal(String),
}

#[derive(Eq, PartialEq, Debug)]
struct Point((usize, usize), usize);

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

struct Maze(HashMap<(usize, usize), Tile>);

impl Maze {
    fn neighbours(&self, xy: &(usize, usize)) -> HashSet<(usize, usize)> {
        let nbours = [
            (xy.0 + 1, xy.1),
            (xy.0 - 1, xy.1),
            (xy.0, xy.1 + 1),
            (xy.0, xy.1 - 1),
        ];
        let mut result: HashSet<(usize, usize)> = nbours
            .iter()
            .filter(|p| self.0.contains_key(*p))
            .cloned()
            .collect();
        if let Some(Tile::Portal(s)) = self.0.get(xy) {
            for (xy2, tile) in self.0.iter() {
                match tile {
                    Tile::Portal(ss) if ((ss.as_str() == s.as_str()) && (xy != xy2)) => {
                        result.insert(*xy2);
                        break;
                    }
                    _ => {}
                };
            }
        }
        result
    }

    fn port(&self, port: &str) -> Option<(usize, usize)> {
        self.0
            .iter()
            .find(|(_, tile)| match tile {
                Tile::Portal(aa) if aa.as_str() == port => true,
                _ => false,
            })
            .map(|(xy, _)| *xy)
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
                        .filter(|(&xy, _)| distance((x, y), xy) == 1)
                        .next()
                    {
                        ports.push(Port {
                            a: *xy,
                            b: (x, y),
                            id: match distance((0, 0), (x, y)) < distance((0, 0), *xy) {
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
        // locate and update portals to the map
        for port in ports {
            let xy = map
                .iter_mut()
                .find(|(xy, _)| distance(port.a, **xy) == 1 || distance(port.b, **xy) == 1)
                .unwrap();
            *xy.1 = Tile::Portal(port.id);
        }

        Ok(Self(map))
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.0.iter().map(|((x, _), _)| x).min().unwrap();
        let min_y = self.0.iter().map(|((_, y), _)| y).min().unwrap();
        let max_x = self.0.iter().map(|((x, _), _)| x).max().unwrap();
        let max_y = self.0.iter().map(|((_, y), _)| y).max().unwrap();

        for y in *min_y..=*max_y {
            for x in *min_x..=*max_x {
                match self.0.get(&(x, y)) {
                    Some(Tile::Open) => {
                        f.write_str(".")?;
                    }
                    None => {
                        f.write_str(" ")?;
                    }
                    Some(Tile::Portal(_)) => {
                        f.write_str("+")?;
                    }
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
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
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut unseen: BinaryHeap<Point> = BinaryHeap::new();
    unseen.push(Point(start, 0));

    let result = loop {
        let current = unseen.pop().unwrap();

        if current.0 == end {
            break current.1;
        }
        seen.insert(current.0);
        let neighbours: Vec<(usize, usize)> = maze
            .neighbours(&current.0)
            .difference(&seen)
            .cloned()
            .collect();
        unseen.extend(neighbours.iter().map(|xy| Point(*xy, 1 + current.1)));
    };

    Ok(result)
}

fn main() -> Result<()> {
    let maze = Maze::from_str(&read_to_string("resources/day20-input.txt")?)?;
    let part1 = solve1(&maze).unwrap();
    println!("part 1: {}", &part1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(0, distance((1, 1), (1, 1)));
        assert_eq!(1, distance((0, 1), (1, 1)));
        assert_eq!(1, distance((1, 1), (1, 0)));
        assert_eq!(2, distance((1, 1), (0, 0)));
        assert_eq!(10, distance((5, 5), (0, 0)));
        assert_eq!(10, distance((0, 0), (5, 5)));
    }

    #[test]
    fn test_neighbours() {
        let maze = Maze::from_str(&read_to_string("resources/day20-test.txt").unwrap()).unwrap();
        // case 1: no portal
        let t1 = maze.neighbours(&(9, 2));
        assert_eq!(t1.len(), 1);

        // case 2: with portal
        let t1 = maze.neighbours(&(9, 6));
        assert_eq!(t1.len(), 2);
    }

    #[test]
    fn test_case1() {
        let maze = Maze::from_str(&read_to_string("resources/day20-test.txt").unwrap()).unwrap();
        let result = solve1(&maze).unwrap();
        assert_eq!(23, result);
    }

    #[test]
    fn test_case2() {
        let maze = Maze::from_str(&read_to_string("resources/day20-test2.txt").unwrap()).unwrap();
        let result = solve1(&maze).unwrap();
        assert_eq!(58, result);
    }
}
