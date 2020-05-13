use anyhow::Result;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_input(fname: &str) -> Result<Vec<u64>> {
    let mut result = Vec::new();

    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let mass = line?.parse::<u64>()?;
        result.push(mass);
    }

    Ok(result)
}

fn solve1(masses: &[u64]) -> Result<u64> {
    let mut result = 0u64;
    masses.iter().for_each(|m| result += m / 3 - 2 as u64);
    Ok(result)
}

fn calculate_fuel(mass: i64, accum: i64) -> i64 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        return calculate_fuel(fuel, accum + fuel);
    }
    accum
}

fn solve2(masses: &[u64]) -> Result<u64> {
    let mut result = 0u64;
    masses
        .iter()
        .for_each(|m| result += calculate_fuel(*m as i64, 0) as u64);
    Ok(result)
}

fn main() -> Result<()> {
    let input = parse_input("resources/day1-input.txt")?;
    println!("part 1: {}", solve1(&input)?);
    println!("part 2: {}", solve2(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(2, solve1(&vec![12]).unwrap());
        assert_eq!(2, solve1(&vec![14]).unwrap());
        assert_eq!(654, solve1(&vec![1969]).unwrap());
        assert_eq!(33583, solve1(&vec![100756]).unwrap());
    }

    #[test]
    fn calculate_fuel_test() {
        assert_eq!(966, calculate_fuel(1969, 0));
        assert_eq!(2, calculate_fuel(14, 0));
        assert_eq!(50346, calculate_fuel(100756, 0));
    }
}
