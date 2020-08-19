use anyhow::Result;
use rayon::prelude::*;

fn parse_input(path: &str) -> Result<Vec<isize>> {
    let mut result = vec![];
    let s = std::fs::read_to_string(path)?;

    for digit in s.trim_end().chars() {
        result.push(
            digit
                .to_digit(10)
                .ok_or_else(|| anyhow::anyhow!("parse error"))? as isize,
        );
    }

    Ok(result)
}

fn phase_parallel(input: &[isize], pattern: &[isize]) -> Result<Vec<isize>> {
    let result = input
        .par_iter()
        .enumerate()
        .map(|(i, _)| {
            let sum: isize = input
                .iter()
                .enumerate()
                .map(|(j, n)| pattern[((j + 1) / (i + 1) % 4) as usize] * n)
                .sum();
            (sum % 10).abs()
        })
        .collect();

    Ok(result)
}

fn solve1(input: &[isize], pattern: &[isize], phases: usize) -> Result<String> {
    let mut input: Vec<isize> = input.iter().cloned().collect();

    for _phase in 0..phases {
        input = phase_parallel(&input, pattern)?;
    }
    let mut result = "".to_string();

    input
        .iter()
        .take(8)
        .for_each(|i| result.push_str(&i.to_string()));

    Ok(result)
}

fn solve2(input: &[isize]) -> Result<String> {
    slet input: Vec<isize> = input
        .iter()
        .cycle()
        .take(input.len() * 10000)
        .cloned()
        .collect();

    let s: String = input[..7].iter().map(|n| n.to_string()).collect();
    let offset = s.parse::<usize>().unwrap();
    let mut output: Vec<isize> = input[offset..].iter().cloned().collect();

    for _ in 0..100 {
        let mut sum = 0;
        for i in 1..=output.len() {
            let i = output.len() - i;
            sum += output[i];
            output[i] = sum % 10;
        }
    }

    let mut result = "".to_string();
    output
        .iter()
        .take(8)
        .for_each(|i| result.push_str(&i.to_string()));

    Ok(result)
}

fn main() -> Result<()> {
    let input = parse_input("resources/day16-input.txt")?;
    let pattern = [0, 1, 0, -1];
    println!("part 1: {}", solve1(&input, &pattern, 100)?);
    println!("part 2: {}", solve2(&input)?);
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_test() {
        let input = [1isize, 2, 3, 4, 5, 6, 7, 8];
        let pattern = [0, 1, 0, -1];
        assert_eq!(
            &[4, 8, 2, 2, 6, 1, 5, 8].to_vec(),
            &phase_parallel(&input, &pattern).unwrap()
        );
        assert_eq!(
            &[3, 4, 0, 4, 0, 4, 3, 8].to_vec(),
            &phase_parallel(&[4, 8, 2, 2, 6, 1, 5, 8], &pattern).unwrap()
        );
    }

    #[test]
    fn solve1_test() {
        let input = [1isize, 2, 3, 4, 5, 6, 7, 8];
        let pattern = [0, 1, 0, -1];
        assert_eq!("01029498", &solve1(&input, &pattern, 4).unwrap());
    }

    #[test]
    fn solve1_test2() {
        let tests = vec![
            ("resources/day16-test.txt", "24176176"),
            ("resources/day16-test2.txt", "73745418"),
            ("resources/day16-test3.txt", "52432133"),
        ];
        let pattern = [0, 1, 0, -1];
        for (path, expected) in tests {
            let input = parse_input(path).unwrap();
            assert_eq!(expected, &solve1(&input, &pattern, 100).unwrap());
        }
    }
}
