use anyhow::Result;
use std::fmt::Display;
use std::fmt::Write;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct Eris {
    bits: Vec<u32>,
}

impl Eris {
    fn rating(&self) -> Result<u64> {
        let s = self.minute_to_string(self.bits.len() - 1)?;
        let mut result = 0;
        for (pos, ch) in s.chars().enumerate() {
            if ch == '1' {
                result += 2u64.pow(pos as u32);
            }
        }
        Ok(result)
    }

    fn minute_to_string(&self, minute: usize) -> Result<String> {
        let mut s = String::from("");
        write!(s, "{:025b}", &self.bits[minute])?;
        let s = s.chars().rev().collect();
        Ok(s)
    }

    fn is_bug(&self, x: isize, y: isize, minute: usize) -> bool {
        if (x < 0) || (x > 4) || (y < 0) || (y > 4) {
            false
        } else {
            let pos = (y * 5) + x;
            (1 << pos as u32) & self.bits[minute] > 0
        }
    }

    fn tick(&mut self) -> bool {
        let last = self.bits.len() - 1;
        let mut bits: u32 = 0;

        for y in 0..5 {
            for x in 0..5 {
                let bugs = [
                    self.is_bug(x - 1, y, last),
                    self.is_bug(x + 1, y, last),
                    self.is_bug(x, y - 1, last),
                    self.is_bug(x, y + 1, last),
                ]
                .iter()
                .filter(|x| **x == true)
                .count();

                let pos = (y * 5) + x;
                match bugs {
                    1 if self.is_bug(x, y, last) => {
                        bits = bits ^ (1 << pos);
                    }
                    1 | 2 if !self.is_bug(x, y, last) => {
                        bits = bits | (1 << pos);
                    }
                    _ => {}
                }
            }
        }
        let result = if self.bits.iter().find(|x| **x == bits).is_some() {
            true
        } else {
            false
        };
        self.bits.push(bits);
        result
    }
}

impl FromStr for Eris {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut bits: u32 = 0;
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    let pos = (y * 5) + x;
                    bits = bits | (1 << pos);
                }
            }
        }
        Ok(Eris { bits: vec![bits] })
    }
}

impl Display for Eris {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (n, _) in self.bits.iter().enumerate() {
            write!(
                f,
                "{}: {}\n",
                n,
                &self.minute_to_string(n).map_err(|_| std::fmt::Error {})?
            )?;
        }
        Ok(())
    }
}

fn solve1(eris: &mut Eris) -> Result<u64> {
    let mut tick = 0;
    loop {
        if eris.tick() {
            return eris.rating();
        }
        tick += 1;
    }
}

fn main() -> Result<()> {
    let mut eris = Eris::from_str(&read_to_string("resources/day24-input.txt")?)?;
    println!("part 1: {}", &solve1(&mut eris)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biodiversity_rating() {
        let s = read_to_string("resources/day24-test.txt").unwrap();
        let eris = Eris::from_str(&s).unwrap();
        assert_eq!(2129920, eris.rating().unwrap());
        assert_eq!(false, eris.is_bug(0, 0, 0));
        assert_eq!(true, eris.is_bug(0, 3, 0));
        assert_eq!(true, eris.is_bug(1, 4, 0));
        assert_eq!(false, eris.is_bug(2, 2, 0));
        assert_eq!(false, eris.is_bug(4, 4, 0));
        assert_eq!(false, eris.is_bug(4, 3, 0));
        assert_eq!(false, eris.is_bug(4, 5, 0));
    }
}
