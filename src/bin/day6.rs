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
    let mut result = 0;

    for path in paths {
        result += path.len();
    }

    Ok(result - paths.len())
}

fn main() -> Result<()> {
    let data = parse_input("resources/day6-input.txt")?;
    let paths = get_paths(&data)?;

    println!("part 1: {:?}", solve1(&paths)?);

    Ok(())
}

