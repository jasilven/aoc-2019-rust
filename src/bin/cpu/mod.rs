use anyhow::Result;
use std::sync::mpsc::{self, Receiver, Sender};

pub fn parse_input(fname: &str) -> Result<Vec<isize>> {
    let input = std::fs::read_to_string(fname)?;
    let input = input.trim_end();
    let result = input.split(',').map(|s| s.parse::<isize>()).collect();
    match result {
        Ok(v) => Ok(v),
        Err(e) => anyhow::bail!("parse error: {}", e),
    }
}

pub struct Cpu {
    pc: usize,
    pub input: Receiver<isize>,
    pub output: Sender<isize>,
    pub prog: Vec<isize>,
}

impl Cpu {
    pub fn new(program: &[isize], input: Receiver<isize>, output: Sender<isize>) -> Cpu {
        Cpu {
            pc: 0,
            input,
            output,
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

    fn get_param(&self, offset: usize, mode: usize) -> Result<isize> {
        match mode {
            0 => Ok(self.prog[self.prog[self.pc + offset] as usize]),
            1 => Ok(self.prog[self.pc + offset]),
            _ => anyhow::bail!("unknown mode: {}", mode),
        }
    }

    pub fn execute(&mut self) -> Result<()> {
        loop {
            match self.parse_instruction()? {
                (1, m1, m2, _) => {
                    let c = self.prog[self.pc + 3] as usize;
                    self.prog[c] =
                        self.get_param(1, m1 as usize)? + self.get_param(2, m2 as usize)?;
                    self.pc += 4;
                }
                (2, m1, m2, _) => {
                    let c = self.prog[self.pc + 3] as usize;
                    self.prog[c] =
                        self.get_param(1, m1 as usize)? * self.get_param(2, m2 as usize)?;
                    self.pc += 4;
                }
                (3, _, _, _) => {
                    let a = self.prog[self.pc + 1] as usize;
                    self.prog[a] = self.input.recv()?;
                    self.pc += 2;
                }
                (4, m1, _, _) => {
                    let a = self.get_param(1, m1 as usize)?;

                    self.output.send(a)?;
                    self.pc += 2;
                }
                (5, m1, m2, _) => {
                    let a = self.get_param(1, m1 as usize)?;
                    let b = self.get_param(2, m2 as usize)?;
                    if a != 0 {
                        self.pc = b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                (6, m1, m2, _) => {
                    let a = self.get_param(1, m1 as usize)?;
                    let b = self.get_param(2, m2 as usize)?;
                    if a == 0 {
                        self.pc = b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                (7, m1, m2, _m3) => {
                    let a = self.get_param(1, m1 as usize)?;
                    let b = self.get_param(2, m2 as usize)?;
                    // let c = self.get_param(3, m3 as usize)? as usize;
                    let c = self.prog[self.pc + 3] as usize;
                    if a < b {
                        self.prog[c] = 1;
                    } else {
                        self.prog[c] = 0;
                    }
                    self.pc += 4;
                }
                (8, m1, m2, _m3) => {
                    let a = self.get_param(1, m1 as usize)?;
                    let b = self.get_param(2, m2 as usize)?;
                    let c = self.prog[self.pc + 3] as usize;
                    if a == b {
                        self.prog[c] = 1;
                    } else {
                        self.prog[c] = 0;
                    }
                    self.pc += 4;
                }
                (99, _, _, _) => break,
                _ => anyhow::bail!("unknown opcode '{}' at '{}'", self.prog[self.pc], &self.pc),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod cpu_tests {
    use super::*;

    #[test]
    fn parse_instruction() {
        let (tx, rx): (Sender<isize>, Receiver<isize>) = mpsc::channel();
        let cpu = Cpu::new(&vec![1002], rx, tx);
        assert_eq!(cpu.parse_instruction().unwrap(), (2, 0, 1, 0));
    }

    #[test]
    fn compare_tests() {
        let progs = vec![
            vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
            vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
        ];

        for prog in progs {
            let (tx, rx): (Sender<isize>, Receiver<isize>) = mpsc::channel();
            let (tx2, rx2): (Sender<isize>, Receiver<isize>) = mpsc::channel();
            let mut cpu = Cpu::new(&prog, rx, tx2);
            tx.send(9).unwrap();
            cpu.execute().unwrap();
            assert_eq!(0, rx2.recv().unwrap());
        }
    }

    #[test]
    fn jump_tests() {
        let progs = vec![
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
        ];

        for prog in progs {
            let (tx, rx): (Sender<isize>, Receiver<isize>) = mpsc::channel();
            let (tx2, rx2): (Sender<isize>, Receiver<isize>) = mpsc::channel();
            let mut cpu = Cpu::new(&prog, rx, tx2);
            tx.send(0).unwrap();
            cpu.execute().unwrap();
            assert_eq!(0, rx2.recv().unwrap());
        }
    }

    #[test]
    fn test_larger_program() {
        let prog = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        let (tx, rx): (Sender<isize>, Receiver<isize>) = mpsc::channel();
        let (tx2, rx2): (Sender<isize>, Receiver<isize>) = mpsc::channel();
        let mut cpu = Cpu::new(&prog, rx, tx2);
        tx.send(7).unwrap();
        cpu.execute().unwrap();
        assert_eq!(999, rx2.recv().unwrap());
    }
}
