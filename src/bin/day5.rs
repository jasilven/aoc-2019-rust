use anyhow::Result;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

mod cpu;
use cpu::Cpu;

fn solve(input: isize) -> Result<isize> {
    let prog = cpu::parse_input("resources/day5-input.txt")?;
    let (tx, receiver): (Sender<isize>, Receiver<isize>) = mpsc::channel();
    let (sender, rx): (Sender<isize>, Receiver<isize>) = mpsc::channel();

    tx.send(input)?;

    let t = thread::spawn(move || {
        let mut cpu = Cpu::new(&prog, sender, receiver);
        cpu.execute()
    });

    loop {
        let response = rx.recv()?;
        if response != 0 {
            match t.join().unwrap() {
                Ok(_) => {
                    return Ok(response);
                }
                Err(_) => anyhow::bail!("cpu execution failed with input: {}", input),
            }
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

    fn new_cpu(fname: &str) -> (Cpu, Sender<isize>, Receiver<isize>) {
        let prog = cpu::parse_input(fname).unwrap();
        let (sender, rx1): (Sender<isize>, Receiver<isize>) = mpsc::channel();
        let (tx2, receiver): (Sender<isize>, Receiver<isize>) = mpsc::channel();

        (Cpu::new(&prog, tx2, rx1), sender, receiver)
    }

    #[test]
    fn test_input() {
        let testval = 9;
        let (mut cpu, sender, receiver) = new_cpu("resources/day5-test.txt");

        sender.send(testval).unwrap();
        cpu.execute().unwrap();

        assert_eq!(testval, receiver.recv().unwrap());
    }

    #[test]
    fn test_input2() {
        let (mut cpu, _, _) = new_cpu("resources/day5-test2.txt");
        cpu.execute().unwrap();
        assert_eq!(99, cpu.prog[4]);
    }
}
