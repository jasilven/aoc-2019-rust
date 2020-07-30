use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_input<'a>(fname: &str) -> Result<HashMap<String, String>> {
    let mut result = HashMap::new();

    let f = File::open(fname)?;

    for line in BufReader::new(f).lines() {
        let line = line?;
        let splits: Vec<&str> = line.split(')').collect();
        result.insert(splits[1].into(), splits[0].into());
    }

    Ok(result)
}

fn get_paths(orbits: &HashMap<String, String>) -> Result<Vec<Vec<String>>> {
    let mut result = vec![];

    for (outer, inner) in orbits.iter() {
        let mut path = vec![outer.to_owned()];
        let mut current = inner;

        loop {
            path.push(current.to_owned());
            if current == "COM" {
                break;
            } else {
                current = orbits
                    .get(current)
                    .ok_or_else(|| anyhow::anyhow!("{} orbits nothing!", current))?;
            }
        }
        result.push(path);
    }

    Ok(result)
}

fn solve1(paths: &Vec<Vec<String>>) -> Result<usize> {
    let sum = paths.iter().fold(0, |acc, p| acc + p.len());
    paths.len();
    Ok(sum - paths.len())
}

fn find_path<'a>(name: &str, paths: &'a Vec<Vec<String>>) -> Result<&'a Vec<String>> {
    paths
        .iter()
        .find(|p| p[0] == name)
        .ok_or_else(|| anyhow::anyhow!("YOU path not found"))
}

fn solve2(paths: &Vec<Vec<String>>) -> Result<usize> {
    let you_path = find_path("YOU", paths)?;
    let san_path = find_path("SAN", paths)?;
    let mut you_rev = you_path.iter().rev();
    let mut san_rev = san_path.iter().rev();

    loop {
        let a = you_rev
            .next()
            .ok_or_else(|| anyhow::anyhow!("YOU path end reached"))?;
        let b = san_rev
            .next()
            .ok_or_else(|| anyhow::anyhow!("SAN path end reached"))?;
        if a != b {
            break;
        }
    }

    Ok(you_rev.len() + san_rev.len())
}

fn main() -> Result<()> {
    let data = parse_input("resources/day6-input.txt")?;
    let paths = get_paths(&data)?;

    println!("part 1: {}", solve1(&paths)?);
    println!("part 2: {}", solve2(&paths)?);

    Ok(())
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn part2_test_data() {
        let data = parse_input("resources/day6-test.txt").unwrap();
        let paths = get_paths(&data).unwrap();

        assert_eq!(4, solve2(&paths).unwrap());
    }
}
