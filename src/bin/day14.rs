use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Reaction {
    rhs_quantity: u64,
    lhs_ingredients: Vec<(String, u64)>,
}

fn parse_input(path: &str) -> Result<HashMap<String, Reaction>> {
    let mut result = HashMap::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let splits: Vec<&str> = line.split(" => ").collect();
        let target: Vec<&str> = splits[1].split(" ").collect();

        let mut ingredients = vec![];
        let components: Vec<&str> = splits[0].split(", ").collect();
        for component in components {
            let ingredient_quantity: Vec<&str> = component.split(" ").collect();
            ingredients.push((
                ingredient_quantity[1].into(),
                ingredient_quantity[0].parse::<u64>()?,
            ));
        }
        let reaction = Reaction {
            rhs_quantity: target[0].parse::<u64>()?,
            lhs_ingredients: ingredients,
        };
        result.insert(target[1].into(), reaction);
    }
    return Ok(result);
}

fn solve1(
    reactions: &HashMap<String, Reaction>,
    remaining: &mut HashMap<String, u64>,
    chemical: &str,
    amount: u64,
    ore_cnt: &mut u64,
) -> Result<()> {
    let mut amount = amount;

    if chemical == "ORE" {
        *ore_cnt += amount;
        return Ok(());
    }

    let remaining_cnt = remaining.entry(chemical.to_string()).or_insert(0);
    if remaining_cnt >= &mut amount {
        *remaining_cnt -= amount;
        return Ok(());
    } else {
        amount -= *remaining_cnt;
        *remaining_cnt = 0;
    }

    let reaction = reactions.get(chemical).unwrap();

    let batch_size = (amount as f64 / reaction.rhs_quantity as f64).ceil() as u64;
    let left_over = batch_size * reaction.rhs_quantity - amount;
    *remaining.entry(chemical.to_string()).or_insert(0) += left_over;

    for chem in reaction.lhs_ingredients.iter() {
        solve1(reactions, remaining, &chem.0, chem.1 * batch_size, ore_cnt)?;
    }

    Ok(())
}

fn solve2(reactions: &HashMap<String, Reaction>) -> Result<u64> {
    let max_ores = 1000000000000;

    let mut start = std::u64::MIN;
    let mut end = std::u64::MAX / 1000000;

    while start <= end {
        let mut ore_count = 0;
        let mid = start + ((end - start) / 2);

        solve1(&reactions, &mut HashMap::new(), "FUEL", mid, &mut ore_count)?;

        if ore_count == max_ores {
            return Ok(mid);
        } else if ore_count > max_ores {
            end = mid - 1;
        } else if ore_count < max_ores {
            start = mid + 1;
        }
    }

    Ok(end)
}

fn main() -> Result<()> {
    let data = parse_input("resources/day14-input.txt")?;
    let mut remaining = HashMap::new();
    let mut ore_cnt = 0;
    solve1(&data, &mut remaining, "FUEL", 1, &mut ore_cnt)?;
    println!("Part 1: {:?}", ore_cnt);
    println!("Part 2: {:?}", solve2(&data)?);

    Ok(())
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn part1_test() {
        let tests = vec![
            ("resources/day14-test.txt", 31),
            ("resources/day14-test2.txt", 165),
            ("resources/day14-test3.txt", 13312),
            ("resources/day14-test4.txt", 180697),
            ("resources/day14-test5.txt", 2210736),
        ];

        for test in tests {
            let data = parse_input(test.0).unwrap();
            let mut remaining = HashMap::new();
            let mut ore_cnt = 0;
            solve1(&data, &mut remaining, "FUEL", 1, &mut ore_cnt).unwrap();
            assert_eq!(test.1, ore_cnt);
        }
    }

    #[test]
    fn part2_test() {
        let tests = vec![
            ("resources/day14-test3.txt", 82892753),
            ("resources/day14-test4.txt", 5586022),
            ("resources/day14-test5.txt", 460664),
        ];

        for test in tests {
            let data = parse_input(test.0).unwrap();
            let mut remaining = HashMap::new();
            let mut ore_cnt1 = 0;
            let mut ore_cnt2 = 0;
            solve1(&data, &mut remaining, "FUEL", test.1, &mut ore_cnt1).unwrap();
            solve1(&data, &mut remaining, "FUEL", test.1 + 1, &mut ore_cnt2).unwrap();
            assert!(ore_cnt1 <= 1000_000_000_000 && ore_cnt2 > 1000_000_000_000);
        }
    }
}
