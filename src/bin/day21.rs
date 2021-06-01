use anyhow::Result;
use std::thread;
mod cpu;
use cpu::Cpu;

fn solve1(prog: &[i128]) -> Result<i128> {
    let (mut cpu, tx, rx) = Cpu::new(&prog);

    let handle = thread::spawn(move || -> Result<()> { cpu.execute() });

    // read prompt
    while rx.recv()? != 10 {}

    // write instructions
    for b in b"NOT C J\nNOT A T\nOR T J\nAND D J\nWALK\n" {
        tx.send(*b as i128)?;
    }
    // read response
    let result = loop {
        match rx.recv() {
            Ok(output) => {
                if output > 128 {
                    break output;
                }
            }
            Err(_) => anyhow::bail!("channel closed"),
        }
    };

    handle.join().expect("join failed").expect("cpu failed");

    Ok(result)
}

fn main() -> Result<()> {
    let prog = cpu::parse_input("resources/day21-input.txt")?;

    println!("part 1: {}", solve1(&prog)?);

    Ok(())
}
