use anyhow::Result;

pub struct Cpu {
    pc: usize,
    pub prog: Vec<isize>,
}

impl Cpu {
    pub fn new(program: &[isize]) -> Cpu {
        Cpu {
            pc: 0,
            prog: program.into(),
        }
    }

    fn parse_instruction(&self) -> Result<(isize, isize, isize, isize)> {
        let s = format!("{}{}", "0000", self.prog[self.pc]);
        let inst: Vec<char> = s.chars().rev().take(5).collect();

        let opstr = format!("{}{}", inst[1], inst[0]);
        let opcode: isize = opstr.parse()?;
        let m1: isize = inst[2] as isize - 48;
        let m2: isize = inst[3] as isize - 48;
        let m3: isize = inst[4] as isize - 48;
        Ok((opcode, m1, m2, m3))
    }

    pub fn execute(&mut self) -> Result<()> {
        loop {
            let a = self.prog[self.pc + 1] as usize;
            let b = self.prog[self.pc + 2] as usize;
            let c = self.prog[self.pc + 3] as usize;
            let (opcode, _m1, _m2, _m3) = self.parse_instruction()?;
            match opcode {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_instruction() {
        let cpu = Cpu::new(&vec![1002]);
        assert_eq!(cpu.parse_instruction().unwrap(), (2, 0, 1, 0));
    }
}
