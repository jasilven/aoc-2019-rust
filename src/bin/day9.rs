use anyhow::Result;
mod cpu;
use cpu::Cpu;

fn solve(prog: &[i128], input: i128) -> Result<i128> {
    let (mut cpu, tx1, rx2) = Cpu::new(&prog);

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
