use anyhow::Result;
use std::collections::HashMap;

mod cpu;
use cpu::Cpu;

fn solve1(prog: &[i128]) -> Result<usize> {
    let mut map: HashMap<(usize, usize), char> = HashMap::new();

    for y in 0..50 {
        for x in 0..50 {
            let (mut cpu, tx, rx) = Cpu::new(&prog);
            tx.send(x)?;
            tx.send(y)?;

            cpu.execute()?;
            map.insert(
                (x as usize, y as usize),
                match rx.recv() {
                    Ok(0) => '.',
                    Ok(1) => '#',
                    Ok(x) => anyhow::bail!("invalid response from cpu: {}", x),
                    Err(e) => anyhow::bail!("recv error: {}", e),
                },
            );
        }
    }

    let result = map.values().filter(|ch| *ch == &'#').count();
    return Ok(result);
}

fn main() -> Result<()> {
    let prog = cpu::parse_input("resources/day19-input.txt")?;

    println!("part 1: {}", solve1(&prog)?);

    Ok(())
}
