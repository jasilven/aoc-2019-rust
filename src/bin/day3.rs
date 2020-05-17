use anyhow::Result;
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

fn main() -> Result<()> {
    let input = parse_input("resources/day3-input.txt")?;
    dbg!(&input[0]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
