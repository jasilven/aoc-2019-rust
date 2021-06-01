use anyhow::Result;
use std::thread;
mod cpu;
use cpu::Cpu;

fn solve(prog: &[i128], instructions: &[u8]) -> Result<i128> {
    let (mut cpu, tx, rx) = Cpu::new(&prog);

    let handle = thread::spawn(move || -> Result<()> { cpu.execute() });

    // read prompt
    while rx.recv()? != 10 {}

    // write instructions
    for b in instructions {
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
    let part1 = b"NOT C J\nNOT A T\nOR T J\nAND D J\nWALK\n";
    let part2 = b"NOT A T\nNOT B J\nOR J T\nNOT C J\nOR J T\nNOT D J\nNOT J J\nAND T J\nAND E T\nOR H T\nAND T J\nRUN\n";
    println!("part 1: {}", solve(&prog, part1)?);
    println!("part 2: {}", solve(&prog, part2)?);

    Ok(())
}
