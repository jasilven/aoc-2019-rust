use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone)]
struct Instruction {
    direction: char,
    steps: u32,
}

fn parse_input(fname: &str) -> Result<Vec<Vec<Instruction>>> {
    let mut result: Vec<Vec<Instruction>> = vec![];

    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        let mut v: Vec<Instruction> = vec![];

        for inst in line.split(',') {
            v.push(Instruction {
                direction: inst
                    .chars()
                    .next()
                    .ok_or(anyhow::anyhow!("unable to parse direction"))?,
                steps: inst[1..].parse::<u32>()?,
            });
        }
        result.push(v);
    }

    Ok(result)
}

fn build_wiremap(instructions: &[Instruction]) -> Result<HashSet<(isize, isize)>> {
    let mut result = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    for inst in instructions.iter() {
        match inst.direction {
            'U' => {
                for _ in 1..=inst.steps {
                    y += 1;
                    result.insert((x, y));
                }
            }
            'D' => {
                for _ in 1..=inst.steps {
                    y -= 1;
                    result.insert((x, y));
                }
            }
            'L' => {
                for _ in 1..=inst.steps {
                    x -= 1;
                    result.insert((x, y));
                }
            }
            'R' => {
                for _ in 1..=inst.steps {
                    x += 1;
                    result.insert((x, y));
                }
            }
            x => anyhow::bail!("unknown direction '{}'", x),
        }
    }

    Ok(result)
}

fn manh_distance(point: &(isize, isize)) -> usize {
    (point.0.abs() + point.1.abs()) as usize
}

fn solve1(instructions: &Vec<Vec<Instruction>>) -> Result<usize> {
    let map1 = build_wiremap(&instructions[0])?;
    let map2 = build_wiremap(&instructions[1])?;

    let closest_intersection = map1
        .intersection(&map2)
        .min_by(|a, b| manh_distance(a).cmp(&manh_distance(b)))
        .ok_or(anyhow::anyhow!("unable to find intersection"))?;

    Ok(manh_distance(&closest_intersection))
}

fn main() -> Result<()> {
    let input = parse_input("resources/day3-input.txt")?;

    println!("part 1: {}", solve1(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input("resources/day3-test.txt").unwrap();
        assert_eq!(6, solve1(&input).unwrap());

        let input = parse_input("resources/day3-test2.txt").unwrap();
        assert_eq!(159, solve1(&input).unwrap());

        let input = parse_input("resources/day3-test3.txt").unwrap();
        assert_eq!(135, solve1(&input).unwrap());
    }
}
