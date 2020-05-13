use anyhow::Result;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_input(fname: &str) -> Result<Vec<u32>> {
    let mut result = Vec::new();

    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let mass = line?.parse::<u32>()?;
        result.push(mass);
    }

    Ok(result)
}

fn solve1(masses: &[u32]) -> Result<u32> {
    let mut result = 0u32;
    masses.iter().for_each(|m| result += m / 3 - 2 as u32);
    Ok(result)
}

fn main() -> Result<()> {
    let input = parse_input("resources/day1-input.txt")?;
    println!("part 1: {}", solve1(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_calculation() {
        assert_eq!(2, solve1(&vec![12]).unwrap());
        assert_eq!(2, solve1(&vec![14]).unwrap());
        assert_eq!(654, solve1(&vec![1969]).unwrap());
        assert_eq!(33583, solve1(&vec![100756]).unwrap());
    }
}
