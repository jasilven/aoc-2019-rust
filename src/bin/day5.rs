use anyhow::Result;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

mod cpu;
use cpu::Cpu;

fn solve(input: isize) -> Result<isize> {
    let prog = cpu::parse_input("resources/day5-input.txt")?;
    let (tx1, rx1): (Sender<isize>, Receiver<isize>) = mpsc::channel();
    let (tx2, rx2): (Sender<isize>, Receiver<isize>) = mpsc::channel();

    tx1.send(input)?;
    let mut cpu = Cpu::new(&prog, rx1, tx2);
    cpu.execute()?;
    loop {
        let response = rx2.recv()?;
        if response != 0 {
            return Ok(response);
        }
    }
}

fn main() -> Result<()> {
    println!("part 1: {}", solve(1)?);
    println!("part 2: {}", solve(5)?);
    Ok(())
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = cpu::parse_input("resources/day5-test.txt").unwrap();
        let testval = 9;
        let (tx1, rx1): (Sender<isize>, Receiver<isize>) = mpsc::channel();
        let (tx2, rx2): (Sender<isize>, Receiver<isize>) = mpsc::channel();

        tx1.send(testval).unwrap();
        let mut cpu = Cpu::new(&input, rx1, tx2);
        cpu.execute().unwrap();

        assert_eq!(testval, rx2.recv().unwrap());
    }

    #[test]
    fn test_input2() {
        let input = cpu::parse_input("resources/day5-test2.txt").unwrap();
        let (_, rx1): (Sender<isize>, Receiver<isize>) = mpsc::channel();
        let (tx2, _): (Sender<isize>, Receiver<isize>) = mpsc::channel();

        let mut cpu = Cpu::new(&input, rx1, tx2);
        cpu.execute().unwrap();

        assert_eq!(99, cpu.prog[4]);
    }
}
