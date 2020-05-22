use anyhow::Result;
use std::collections::HashSet;

fn is_valid(num: usize) -> bool {
    let mut same_adjacents = false;
    let mut inc_order = true;

    let vec: Vec<char> = num.to_string().chars().collect();
    vec.windows(2).for_each(|chs| {
        if chs[0] == chs[1] {
            same_adjacents = true;
        }
        if (chs[0] as u8) > (chs[1] as u8) {
            inc_order = false;
        }
    });

    same_adjacents && inc_order && true
}

fn is_valid2(num: usize) -> bool {
    let chs: Vec<char> = num.to_string().chars().collect();

    let pairs = chs
        .windows(2)
        .filter(|chs| chs[0] == chs[1])
        .map(|chs| chs[0])
        .collect::<HashSet<char>>();
    let triples = chs
        .windows(3)
        .filter(|chs| (chs[0] == chs[1]) & (chs[0] == chs[2]))
        .map(|chs| chs[0])
        .collect::<HashSet<char>>();

    pairs.difference(&triples).count() != 0
}

fn solve1(start: usize, end: usize) -> Result<Vec<usize>> {
    let mut nums = vec![];
    for num in start..=end {
        if is_valid(num) {
            nums.push(num);
        }
    }

    Ok(nums)
}

fn solve2(start: usize, end: usize) -> Result<Vec<usize>> {
    let nums = solve1(start, end)?;
    let result = nums.iter().filter(|n| is_valid2(**n)).cloned().collect();
    Ok(result)
}

fn main() -> Result<()> {
    let part1 = solve1(353096, 843212)?;
    let part2 = solve2(353096, 843212)?;
    println!("part 1: {}", part1.len());
    println!("part 2: {}", part2.len());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid() {
        assert_eq!(true, is_valid(111111));
        assert_eq!(false, is_valid(223450));
        assert_eq!(false, is_valid(123789));
    }

    #[test]
    fn test_valid2() {
        assert_eq!(true, is_valid2(112233));
        assert_eq!(false, is_valid2(123444));
        assert_eq!(true, is_valid2(111122));
    }
}
