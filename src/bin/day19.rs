use anyhow::Result;
use std::collections::{HashSet, VecDeque};

mod cpu;
use cpu::Cpu;

fn build_beam(prog: &[i128], size: usize) -> Result<HashSet<(usize, usize)>> {
    let mut set = HashSet::new();

    for y in 0..size {
        for x in 0..size {
            let (mut cpu, tx, rx) = Cpu::new(&prog);

            tx.send(x as i128)?;
            tx.send(y as i128)?;
            cpu.execute()?;

            match rx.recv() {
                Ok(1) => {
                    print!("#");
                    set.insert((x as usize, y as usize));
                }
                Ok(0) => print!(" "),
                Ok(x) => anyhow::bail!("invalid response: {}", x),
                Err(e) => anyhow::bail!("recv error: {}", e),
            }
        }
        println!("");
    }

    return Ok(set);
}

fn solve1(prog: &[i128]) -> Result<usize> {
    let set = build_beam(prog, 50)?;
    Ok(set.len())
}

fn solve2(prog: &[i128]) -> Result<usize> {
    let deltas = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];

    let mut seen = HashSet::new();
    let mut points_to_visit = VecDeque::new();
    seen.insert((5isize, 4isize));
    points_to_visit.push_back((5isize, 4isize));

    loop {
        let point = points_to_visit
            .pop_front()
            .ok_or_else(|| anyhow::anyhow!("no more points to visit"))?;

        for xy in deltas.iter().map(|(dx, dy)| (point.0 + dx, point.1 + dy)) {
            if seen.contains(&xy) {
                continue;
            }

            let (mut cpu, tx, rx) = Cpu::new(prog);
            tx.send(xy.0 as i128)?;
            tx.send(xy.1 as i128)?;
            cpu.execute()?;

            match rx.recv() {
                Ok(1) => {
                    if seen.contains(&(xy.0 - 99 as isize, xy.1 as isize))
                        && seen.contains(&(xy.0, xy.1 - 99))
                        && seen.contains(&(xy.0 - 99, xy.1 - 99))
                    {
                        return Ok(((xy.0 - 99) * 10000 + xy.1 - 99) as usize);
                    }
                    points_to_visit.push_back(xy);
                    seen.insert(xy);
                }
                Ok(0) => {}
                Ok(x) => anyhow::bail!("invalid response: {}", x),
                Err(e) => anyhow::bail!("recv error: {}", e),
            }
        }
    }
}

fn main() -> Result<()> {
    let prog = cpu::parse_input("resources/day19-input.txt")?;
    println!("part 1: {}", solve1(&prog)?);
    println!("part 2: {}", solve2(&prog)?);

    Ok(())
}
