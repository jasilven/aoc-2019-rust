use anyhow::Result;

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

    same_adjacents && inc_order
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

fn main() -> Result<()> {
    let part1 = solve1(353096, 843212)?;
    println!("part 1: {}", part1.len());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(true, is_valid(111111));
        assert_eq!(false, is_valid(223450));
        assert_eq!(false, is_valid(123789));
    }
}
