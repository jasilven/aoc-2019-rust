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

fn wirepath(instructions: &[Instruction]) -> Result<Vec<(isize, isize)>> {
    let mut result = Vec::new();
    let mut x = 0;
    let mut y = 0;

    for inst in instructions.iter() {
        match inst.direction {
            'U' => {
                for _ in 1..=inst.steps {
                    y += 1;
                    result.push((x, y));
                }
            }
            'D' => {
                for _ in 1..=inst.steps {
                    y -= 1;
                    result.push((x, y));
                }
            }
            'L' => {
                for _ in 1..=inst.steps {
                    x -= 1;
                    result.push((x, y));
                }
            }
            'R' => {
                for _ in 1..=inst.steps {
                    x += 1;
                    result.push((x, y));
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

fn path_distance(point: &(isize, isize), path: &[(isize, isize)]) -> usize {
    let mut result = 0usize;
    for p in path {
        result += 1;
        if point == p {
            break;
        }
    }
    result
}

fn solve1(instructions: &Vec<Vec<Instruction>>) -> Result<usize> {
    let set1: HashSet<(isize, isize)> = wirepath(&instructions[0])?.iter().cloned().collect();
    let set2: HashSet<(isize, isize)> = wirepath(&instructions[1])?.iter().cloned().collect();

    let closest_intersection = set1
        .intersection(&set2)
        .min_by(|a, b| manh_distance(a).cmp(&manh_distance(b)))
        .ok_or(anyhow::anyhow!("unable to find intersection"))?;

    Ok(manh_distance(&closest_intersection))
}

fn solve2(instructions: &Vec<Vec<Instruction>>) -> Result<usize> {
    let path1 = wirepath(&instructions[0])?;
    let path2 = wirepath(&instructions[1])?;
    let set1: HashSet<(isize, isize)> = path1.iter().cloned().collect();
    let set2: HashSet<(isize, isize)> = path2.iter().cloned().collect();

    let closest_intersection = set1
        .intersection(&set2)
        .min_by(|a, b| {
            let d1 = path_distance(a, &path1) + path_distance(a, &path2);
            let d2 = path_distance(b, &path1) + path_distance(b, &path2);
            d1.cmp(&d2)
        })
        .ok_or(anyhow::anyhow!("unable to find intersection"))?;

    Ok(path_distance(closest_intersection, &path1) + path_distance(closest_intersection, &path2))
}

fn main() -> Result<()> {
    let input = parse_input("resources/day3-input.txt")?;

    println!("part 1: {}", solve1(&input)?);
    println!("part 2: {}", solve2(&input)?);

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

    #[test]
    fn test_part2() {
        let input = parse_input("resources/day3-test.txt").unwrap();
        assert_eq!(30, solve2(&input).unwrap());

        let input = parse_input("resources/day3-test2.txt").unwrap();
        assert_eq!(159, solve1(&input).unwrap());

        let input = parse_input("resources/day3-test3.txt").unwrap();
        assert_eq!(135, solve1(&input).unwrap());
    }
}
