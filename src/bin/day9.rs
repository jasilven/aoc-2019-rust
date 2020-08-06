use anyhow::Result;
use std::sync::mpsc::{self, Receiver, Sender};
mod cpu;
use cpu::Cpu;

fn solve(prog: &[i128], input: i128) -> Result<i128> {
    let (tx1, rx): (Sender<i128>, Receiver<i128>) = mpsc::channel();
    let (tx2, rx2): (Sender<i128>, Receiver<i128>) = mpsc::channel();
    let mut cpu = Cpu::new(&prog, tx2, rx);

    tx1.send(input)?;
    cpu.execute()?;

    Ok(rx2.recv()?)
}

fn main() -> Result<()> {
    let prog = cpu::parse_input("resources/day9-input.txt")?;

    println!("part 1: {}", solve(&prog, 1)?);
    println!("part 2: {}", solve(&prog, 2)?);

    Ok(())
}
