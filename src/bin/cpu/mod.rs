use anyhow::Result;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

#[allow(dead_code)]
pub fn parse_input(fname: &str) -> Result<Vec<i128>> {
    let input = std::fs::read_to_string(fname)?;
    let input = input.trim_end();
    let result = input.split(',').map(|s| s.parse::<i128>()).collect();
    match result {
        Ok(v) => Ok(v),
        Err(e) => anyhow::bail!("parse error: {}", e),
    }
}

pub struct Cpu {
    pc: u128,
    base: i128,
    sender: Sender<i128>,
    recver: Receiver<i128>,
    mem: HashMap<u128, i128>,
}

impl Cpu {
    #[allow(dead_code)]
    pub fn new(program: &[i128]) -> (Cpu, Sender<i128>, Receiver<i128>) {
        let (tx, recver): (Sender<i128>, Receiver<i128>) = channel();
        let (sender, rx): (Sender<i128>, Receiver<i128>) = channel();

        (Cpu::new_with_send_recv(program, sender, recver), tx, rx)
    }

    pub fn new_with_send_recv(
        program: &[i128],
        sender: Sender<i128>,
        recver: Receiver<i128>,
    ) -> Cpu {
        let mut hm: HashMap<u128, i128> = HashMap::new();
        for (k, i) in program.iter().enumerate() {
            hm.insert(k as u128, *i);
        }

        Cpu {
            pc: 0u128,
            base: 0,
            sender,
            recver,
            mem: hm,
        }
    }

    pub fn get_mem(&self, k: u128) -> Result<i128> {
        match self.mem.get(&k) {
            Some(val) => Ok(*val),
            None => Ok(0),
        }
    }

    fn set_mem(&mut self, k: u128, v: i128) {
        self.mem.insert(k, v);
    }

    fn parse_instruction(&self) -> Result<(i128, i128, i128, i128)> {
        let s = format!("{}{}", "0000", self.get_mem(self.pc)?);
        let inst: Vec<char> = s.chars().rev().take(5).collect();

        let opstr = format!("{}{}", inst[1], inst[0]);
        let opcode: i128 = opstr.parse()?;
        let m1: i128 = inst[2] as i128 - 48;
        let m2: i128 = inst[3] as i128 - 48;
        let m3: i128 = inst[4] as i128 - 48;
        Ok((opcode, m1, m2, m3))
    }

    fn get_param(&self, offset: i128, mode: u128) -> Result<i128> {
        match mode {
            0 => Ok(self.get_mem(self.get_mem((self.pc as i128 + offset) as u128)? as u128)?),
            1 => Ok(self.get_mem((self.pc as i128 + offset) as u128)?),
            2 => Ok(self.get_mem(
                (self.base + self.get_mem((self.pc as i128 + offset) as u128)?) as u128,
            )?),
            _ => anyhow::bail!("unknown mode: {}", mode),
        }
    }

    pub fn execute(&mut self) -> Result<()> {
        loop {
            match self.parse_instruction()? {
                (1, m1, m2, m3) => {
                    let mut c = self.get_mem(self.pc + 3)?;
                    if m3 == 2 {
                        c = self.base + c;
                    }
                    self.set_mem(
                        c as u128,
                        self.get_param(1, m1 as u128)? + self.get_param(2, m2 as u128)?,
                    );
                    self.pc += 4;
                }
                (2, m1, m2, m3) => {
                    let mut c = self.get_mem(self.pc + 3)?;
                    if m3 == 2 {
                        c = self.base + c;
                    }
                    self.set_mem(
                        c as u128,
                        self.get_param(1, m1 as u128)? * self.get_param(2, m2 as u128)?,
                    );
                    self.pc += 4;
                }
                (3, m1, _, _) => {
                    let mut a = self.get_mem(self.pc + 1)?;
                    if m1 == 2 {
                        a = self.base + a;
                    }
                    self.set_mem(a as u128, self.recver.recv()?);
                    self.pc += 2;
                }
                (4, m1, _, _) => {
                    let a = self.get_param(1, m1 as u128)?;

                    self.sender.send(a)?;
                    self.pc += 2;
                }
                (5, m1, m2, _) => {
                    let a = self.get_param(1, m1 as u128)?;
                    let b = self.get_param(2, m2 as u128)?;
                    if a != 0 {
                        self.pc = b as u128;
                    } else {
                        self.pc += 3;
                    }
                }
                (6, m1, m2, _) => {
                    let a = self.get_param(1, m1 as u128)?;
                    let b = self.get_param(2, m2 as u128)?;
                    if a == 0 {
                        self.pc = b as u128;
                    } else {
                        self.pc += 3;
                    }
                }
                (7, m1, m2, m3) => {
                    let a = self.get_param(1, m1 as u128)?;
                    let b = self.get_param(2, m2 as u128)?;

                    let mut c = self.get_mem(self.pc + 3)?;

                    if m3 == 2 {
                        c = self.base + c;
                    }
                    if a < b {
                        self.set_mem(c as u128, 1);
                    } else {
                        self.set_mem(c as u128, 0);
                    }
                    self.pc += 4;
                }
                (8, m1, m2, m3) => {
                    let a = self.get_param(1, m1 as u128)?;
                    let b = self.get_param(2, m2 as u128)?;
                    let mut c = self.get_mem(self.pc + 3)?;

                    if m3 == 2 {
                        c = self.base + c;
                    }
                    if a == b {
                        self.set_mem(c as u128, 1);
                    } else {
                        self.set_mem(c as u128, 0);
                    }
                    self.pc += 4;
                }
                (9, m1, _, _) => {
                    let a = self.get_param(1, m1 as u128)?;
                    self.base = self.base as i128 + a;
                    self.pc += 2;
                }
                (99, _, _, _) => break,
                _ => anyhow::bail!(
                    "unknown opcode '{}' at '{}'",
                    self.get_mem(self.pc)?,
                    &self.pc
                ),
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
        let (tx, rx): (Sender<i128>, Receiver<i128>) = channel();
        let cpu = Cpu::new_with_send_recv(&vec![1002], tx, rx);
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
            let (tx, rx): (Sender<i128>, Receiver<i128>) = channel();
            let (tx2, rx2): (Sender<i128>, Receiver<i128>) = channel();
            let mut cpu = Cpu::new_with_send_recv(&prog, tx2, rx);
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
            let (tx, rx): (Sender<i128>, Receiver<i128>) = channel();
            let (tx2, rx2): (Sender<i128>, Receiver<i128>) = channel();
            let mut cpu = Cpu::new_with_send_recv(&prog, tx2, rx);
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

        let (tx, rx): (Sender<i128>, Receiver<i128>) = channel();
        let (tx2, rx2): (Sender<i128>, Receiver<i128>) = channel();
        let mut cpu = Cpu::new_with_send_recv(&prog, tx2, rx);
        tx.send(7).unwrap();
        cpu.execute().unwrap();
        assert_eq!(999, rx2.recv().unwrap());
    }

    #[test]
    fn relative_base() {
        let prog: Vec<i128> = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let (_, rx): (Sender<i128>, Receiver<i128>) = channel();
        let (tx2, rx2): (Sender<i128>, Receiver<i128>) = channel();
        let mut cpu = Cpu::new_with_send_recv(&prog, tx2, rx);

        cpu.execute().unwrap();

        let mut output = vec![];
        for _ in 0..prog.len() {
            match rx2.recv() {
                Ok(val) => output.push(val),
                Err(_) => break,
            }
        }
        assert_eq!(&prog, &output);
    }

    #[test]
    fn relative_base2() {
        let prog: Vec<i128> = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        let (_, rx): (Sender<i128>, Receiver<i128>) = channel();
        let (tx2, rx2): (Sender<i128>, Receiver<i128>) = channel();
        let mut cpu = Cpu::new_with_send_recv(&prog, tx2, rx);

        cpu.execute().unwrap();
        let output = rx2.recv().unwrap();
        let digit_cnt = output.to_string().chars().count();
        assert_eq!(16, digit_cnt);
    }

    #[test]
    fn relative_base3() {
        let prog: Vec<i128> = vec![104, 1125899906842624, 99];

        let (_, rx): (Sender<i128>, Receiver<i128>) = channel();
        let (tx2, rx2): (Sender<i128>, Receiver<i128>) = channel();
        let mut cpu = Cpu::new_with_send_recv(&prog, tx2, rx);

        cpu.execute().unwrap();
        let output = rx2.recv().unwrap();
        assert_eq!(1125899906842624, output);
    }
}
