use anyhow::Result;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufRead};

enum Shuffle {
    DealNew,
    Cut(isize),
    DealInc(usize),
}

fn parse_shuffles(fname: &str) -> Result<Vec<Shuffle>> {
    let mut result = vec![];
    let file = File::open(fname)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("Result") {
            break;
        }
        let splits: Vec<String> = line
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        match splits[1].as_str() {
            "with" => result.push(Shuffle::DealInc(splits[3].parse::<usize>()?)),
            "into" => result.push(Shuffle::DealNew),
            s => result.push(Shuffle::Cut(s.parse::<isize>()?)),
        }
    }

    Ok(result)
}

fn solve1(deck_size: usize, shuffles: &[Shuffle], card: &usize) -> Result<usize> {
    let mut deck1: Vec<usize> = Vec::with_capacity(deck_size);
    let mut deck2: Vec<usize> = Vec::with_capacity(deck_size);
    (0..deck_size).for_each(|n| {
        deck1.push(n);
        deck2.push(n);
    });

    let mut deck = &mut deck1;
    let mut other_deck = &mut deck2;

    for shuffle in shuffles {
        match shuffle {
            Shuffle::DealNew => deck.reverse(),
            Shuffle::Cut(mut num) => {
                if num < 0 {
                    num = (deck.len() as isize) + num;
                }
                let mut pos = 0;
                let start: usize = usize::try_from(num)?;
                for n in start..deck.len() + start {
                    other_deck[pos] = deck[n % deck.len()];
                    pos += 1;
                }
                let tmp: &mut Vec<usize> = deck;
                deck = other_deck;
                other_deck = tmp;
            }
            Shuffle::DealInc(num) => {
                for n in 0..deck.len() {
                    let m = (n * num) % deck.len();
                    other_deck[m] = deck[n];
                }
                let tmp: &mut Vec<usize> = deck;
                deck = other_deck;
                other_deck = tmp;
            }
        }
    }
    let mut result = 0;
    for (i, n) in deck.iter().enumerate() {
        if n == card {
            result = i;
        }
    }

    Ok(result)
}

fn modinv(mut a: i128, mut base: i128) -> i128 {
    if base == 1 {
        return 0;
    }

    let orig = base;
    let mut x = 1;
    let mut y = 0;

    while a > 1 {
        let q = a / base;
        let tmp = base;
        base = a % base;
        a = tmp;
        let tmp = y;
        y = x - q * y;
        x = tmp;
    }

    if x < 0 {
        x + orig
    } else {
        x
    }
}

fn modp(b: i128, exp: i128, base: i128) -> i128 {
    let mut x = 1;
    let mut p = b % base;

    for i in 0..128 {
        if 1 & (exp >> i) == 1 {
            x = x * p % base;
        }
        p = p * p % base;
    }

    x
}

fn solve2(deck_size: i128, shuffles: &[Shuffle], times: i128) -> Result<i128> {
    const INDEX: i128 = 2020;
    let mut a = 1;
    let mut b = 0;

    for shuffle in shuffles.iter().rev() {
        match shuffle {
            Shuffle::DealNew => {
                b += 1;
                b *= -1;
                a *= -1;
            }
            Shuffle::Cut(n) => {
                let n = *n as i128;
                b += if n < 0 { n + deck_size } else { n };
            }
            Shuffle::DealInc(n) => {
                let inv = modinv(*n as i128, deck_size);
                a = a * inv % deck_size;
                b = b * inv % deck_size;
            }
        }

        a %= deck_size;
        b %= deck_size;

        if a < 0 {
            a += deck_size;
        }

        if b < 0 {
            b += deck_size;
        }
    }

    let i1 = modp(a, times, deck_size) * INDEX % deck_size;
    let i2 = (modp(a, times, deck_size) + deck_size - 1) % deck_size;
    let i3 = b * i2 % deck_size;
    let i4 = modp(a - 1, deck_size - 2, deck_size);
    let ans = (i1 + i3 * i4) % deck_size;

    Ok(ans)
}

fn main() -> Result<()> {
    let shuffles = parse_shuffles("resources/day22-input.txt")?;
    println!("part 1: {}", solve1(10007, &shuffles, &2019)?);
    println!(
        "part 2: {}",
        solve2(119315717514047, &shuffles, 101741582076661)?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
        let t1 = parse_shuffles("resources/day22-test1.txt").unwrap();
        let t2 = parse_shuffles("resources/day22-test2.txt").unwrap();
        let t3 = parse_shuffles("resources/day22-test3.txt").unwrap();
        let t4 = parse_shuffles("resources/day22-test4.txt").unwrap();
        assert_eq!(0, solve1(10, &t1, &0).unwrap());
        assert_eq!(1, solve1(10, &t2, &0).unwrap());
        assert_eq!(2, solve1(10, &t3, &0).unwrap());
        assert_eq!(7, solve1(10, &t4, &0).unwrap());
    }
}
