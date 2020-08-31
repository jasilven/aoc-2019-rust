use anyhow::Result;
use std::thread;

mod cpu;
use cpu::Cpu;

fn solve(input: i128) -> Result<i128> {
    let prog = cpu::parse_input("resources/day5-input.txt")?;

    let (mut cpu, tx, rx) = Cpu::new(&prog);
    tx.send(input)?;

    let t = thread::spawn(move || cpu.execute());

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
    use std::sync::mpsc::{Receiver, Sender};

    fn new_cpu(fname: &str) -> (Cpu, Sender<i128>, Receiver<i128>) {
        let prog = cpu::parse_input(fname).unwrap();

        Cpu::new(&prog)
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
        assert_eq!(99, cpu.get_mem(4).unwrap());
    }
}
