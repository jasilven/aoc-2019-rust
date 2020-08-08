use anyhow::Result;
mod cpu;
use cpu::Cpu;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn run(prog: &[i128], start_panel: u8) -> Result<HashMap<(isize, isize), u8>> {
    let (tx, rx): (Sender<i128>, Receiver<i128>) = channel();
    let (tx2, rx2): (Sender<i128>, Receiver<i128>) = channel();
    let mut cpu = Cpu::new(prog, tx2, rx);

    let mut curpos = (0, 0);
    let mut direction = 0;
    let mut panels: HashMap<(isize, isize), u8> = HashMap::new();

    let th = thread::spawn(move || cpu.execute());

    loop {
        match panels.get(&curpos).unwrap_or(&start_panel) {
            0 => tx.send(0)?,
            _ => tx.send(1)?,
        }
        match rx2.recv() {
            Ok(x) => panels.insert(curpos, x as u8),
            Err(_) => break,
        };

        match (direction, rx2.recv()?) {
            (0, 0) => {
                curpos = (curpos.0 - 1, curpos.1);
                direction = 3;
            }
            (0, 1) => {
                curpos = (curpos.0 + 1, curpos.1);
                direction = 1;
            }
            (1, 0) => {
                curpos = (curpos.0, curpos.1 - 1);
                direction = 0;
            }
            (1, 1) => {
                curpos = (curpos.0, curpos.1 + 1);
                direction = 2;
            }
            (2, 0) => {
                curpos = (curpos.0 + 1, curpos.1);
                direction = 1;
            }
            (2, 1) => {
                curpos = (curpos.0 - 1, curpos.1);
                direction = 3;
            }
            (3, 0) => {
                curpos = (curpos.0, curpos.1 + 1);
                direction = 2;
            }
            (3, 1) => {
                curpos = (curpos.0, curpos.1 - 1);
                direction = 0;
            }
            (dir, turn) => anyhow::bail!("Unknown direction/turn: {}/{}", dir, turn),
        }
    }

    th.join().expect("Can't join thread")?;

    Ok(panels)
}

fn solve1(prog: &[i128]) -> Result<usize> {
    let panels = run(prog, 0)?;
    Ok(panels.len())
}

fn solve2(prog: &[i128]) -> Result<()> {
    let panels = run(prog, 1)?;
    let max_x = panels.keys().max_by_key(|(x, _)| x).unwrap().0;
    let min_x = panels.keys().min_by_key(|(x, _)| x).unwrap().0;
    let max_y = panels.keys().max_by_key(|(_, y)| y).unwrap().1;
    let min_y = panels.keys().min_by_key(|(_, y)| y).unwrap().1;

    dbg!(min_y, max_x, min_y, max_y);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match panels.get(&(x, y)) {
                Some(0) | None => print!("."),
                Some(1) => print!("#"),
                Some(c) => anyhow::bail!("Unexpected color {:?}", c),
            };
        }
        println!("");
    }
    Ok(())
}

fn main() -> Result<()> {
    let prog = cpu::parse_input("resources/day11-input.txt")?;
    println!("part 1: {:?}", solve1(&prog)?);
    println!("part 2:");

    solve2(&prog)?;

    // ...##.#..#..##..###..###...##...##..#..#...
    // ....#.#..#.#..#.#..#.#..#.#..#.#..#.#..#...
    // ....#.####.#..#.#..#.###..#....#....#..#...
    // ....#.#..#.####.###..#..#.#.##.#....#..#...
    // .#..#.#..#.#..#.#.#..#..#.#..#.#..#.#..#...
    // ..##..#..#.#..#.#..#.###...###..##...##....

    Ok(())
}
