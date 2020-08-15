use anyhow::Result;

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

fn expand_pattern(index: usize, pattern: &[isize]) -> Vec<isize> {
    let mut result = vec![];
    for digit in pattern.iter() {
        for i in 0..index {
            result.push(*digit);
        }
    }
    result
}

fn phase(input: &[isize], pattern: &[isize]) -> Result<Vec<isize>> {
    let mut result = vec![];

    for (index, _) in input.iter().enumerate() {
        let mut sum = 0;
        let pat: Vec<isize> = expand_pattern(index + 1, pattern)
            .iter()
            .cloned()
            .cycle()
            .take(input.len() + 1)
            .skip(1)
            .collect();
        for (index, val) in input.iter().enumerate() {
            sum += pat[index] * val;
        }
        let digit = sum
            .to_string()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap();
        result.push(digit as isize);
    }

    Ok(result)
}

fn solve1(input: &[isize], pattern: &[isize], phases: usize) -> Result<String> {
    let mut input: Vec<isize> = input.iter().cloned().collect();

    for _phase in 0..phases {
        input = phase(&input, pattern)?;
    }
    let mut result = "".to_string();

    input
        .iter()
        .take(8)
        .for_each(|i| result.push_str(&i.to_string()));

    Ok(result)
}

fn main() -> Result<()> {
    let input = parse_input("resources/day16-input.txt")?;
    let pattern = [0, 1, 0, -1];

    println!("part 1: {}", solve1(&input, &pattern, 100)?);
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_pattern_test() {
        let pat = [0, 1, 0, -1];
        let expanded_pattern = expand_pattern(3, &pat);

        assert_eq!(12, expand_pattern(3, &pat).len());
        for (k, v) in [0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1].iter().enumerate() {
            assert_eq!(*v, expanded_pattern[k]);
        }
    }

    #[test]
    fn phase_test() {
        let input = [1isize, 2, 3, 4, 5, 6, 7, 8];
        let pattern = [0, 1, 0, -1];
        assert_eq!(
            &[4, 8, 2, 2, 6, 1, 5, 8].to_vec(),
            &phase(&input, &pattern).unwrap()
        );
        assert_eq!(
            &[3, 4, 0, 4, 0, 4, 3, 8].to_vec(),
            &phase(&[4, 8, 2, 2, 6, 1, 5, 8], &pattern).unwrap()
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
