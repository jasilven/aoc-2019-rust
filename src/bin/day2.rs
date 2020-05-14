use anyhow::Result;

struct Cpu {
    pc: usize,
    prog: Vec<isize>,
}

impl Cpu {
    fn new(program: &[isize]) -> Cpu {
        Cpu {
            pc: 0,
            prog: program.into(),
        }
    }

    fn execute(&mut self) -> Result<()> {
        loop {
            let a = self.prog[self.pc + 1] as usize;
            let b = self.prog[self.pc + 2] as usize;
            let c = self.prog[self.pc + 3] as usize;

            match self.prog[self.pc] {
                1 => self.prog[c] = self.prog[a] + self.prog[b],
                2 => self.prog[c] = self.prog[a] * self.prog[b],
                99 => break,
                _ => anyhow::bail!("unknown opcode '{}' at '{}'", self.prog[self.pc], &self.pc),
            }
            self.pc += 4;
        }
        Ok(())
    }
}

fn parse_input(fname: &str) -> Result<Vec<isize>> {
    let input = std::fs::read_to_string(fname)?;
    let input = input.trim_end();
    let result = input.split(',').map(|s| s.parse::<isize>()).collect();
    match result {
        Ok(v) => Ok(v),
        Err(e) => anyhow::bail!("parse error: {}", e),
    }
}

fn solve1(mut input: Vec<isize>) -> Result<isize> {
    input[1] = 12;
    input[2] = 2;
    let mut cpu = Cpu::new(&input);
    cpu.execute()?;

    Ok(cpu.prog[0])
}

fn main() -> Result<()> {
    let input = parse_input("resources/day2-input.txt")?;
    println!("part 1: {}", solve1(input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let input = parse_input("resources/day2-test.txt").unwrap();
        assert_eq!(3500, solve1(input).unwrap());
    }
}
