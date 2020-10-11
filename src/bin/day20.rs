use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let dcol = a.0.max(b.0) - a.0.min(b.0);
    let drow = a.1.max(b.1) - a.1.min(b.1);
    drow.max(dcol) + drow.min(dcol)
}

fn parse_input(fname: &str) -> Result<(HashMap<(usize, usize), Tile>, Vec<Port>)> {
    let mut map = HashMap::new();
    let mut ports = vec![];
    let mut letters: HashMap<(usize, usize), char> = HashMap::new();
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    for (row, line) in reader.lines().enumerate() {
        let line = line?;
        for (col, ch) in line.chars().enumerate() {
            if ch == '.' {
                map.insert((col, row), Tile::Open);
            } else if ch.is_alphabetic() {
                if let Some((xy, ch2)) = letters
                    .iter()
                    .filter(|(&xy, _)| distance((col, row), xy) == 1)
                    .next()
                {
                    ports.push(Port {
                        a: *xy,
                        b: (col, row),
                        id: match distance((0, 0), (col, row)) < distance((0, 0), *xy) {
                            true => format!("{}{}", ch, ch2),
                            _ => format!("{}{}", ch2, ch),
                        },
                    });
                } else {
                    letters.insert((col, row), ch);
                }
            }
        }
    }

    Ok((map, ports))
}

fn merge_ports(map: &mut HashMap<(usize, usize), Tile>, ports: Vec<Port>) {
    for port in ports {
        let xy = map
            .iter_mut()
            .find(|(xy, _)| distance(port.a, **xy) == 1 || distance(port.b, **xy) == 1)
            .unwrap();
        *xy.1 = Tile::Portal(port.id);
    }
}

fn main() -> Result<()> {
    let (mut map, ports) = parse_input("resources/day20-test.txt")?;
    merge_ports(&mut map, ports);
    println!("{:?}", &map);

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
}
