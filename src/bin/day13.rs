use anyhow::Result;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
mod cpu;
use cpu::Cpu;

fn print_tiles(triples: &HashMap<(i128, i128), char>) -> Result<()> {
    let max_x = triples.keys().max_by_key(|(x, _)| x).unwrap().0;
    let min_x = triples.keys().min_by_key(|(x, _)| x).unwrap().0;
    let max_y = triples.keys().max_by_key(|(_, y)| y).unwrap().1;
    let min_y = triples.keys().min_by_key(|(_, y)| y).unwrap().1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match triples
                .get(&(x, y))
                .ok_or_else(|| anyhow::anyhow!("not found"))
            {
                Ok(ch) => print!("{}", ch),
                Err(err) => anyhow::bail!(err),
            }
        }
        println!("");
    }
    Ok(())
}

fn solve1(prog: &[i128]) -> Result<HashMap<(i128, i128), char>> {
    let mut result = HashMap::new();
    let (_tx, rx): (Sender<i128>, Receiver<i128>) = channel();
    let (tx2, rx2): (Sender<i128>, Receiver<i128>) = channel();

    let mut cpu = Cpu::new(prog, tx2, rx);
    let th = thread::spawn(move || cpu.execute());

    loop {
        let mut x = -1;
        match rx2.recv() {
            Ok(val) => x = val,
            Err(_) => break,
        };
        let y = rx2.recv()?;
        match rx2.recv() {
            Ok(val) => {
                let tile = match val {
                    0 => ' ',
                    1 => '#',
                    2 => '▢',
                    3 => '▬',
                    4 => '●',
                    _ => anyhow::bail!("received unsupported block!"),
                };
                result.insert((x, y), tile);
            }
            Err(_) => anyhow::bail!("unexpexted end of output from cpu"),
        }
    }

    th.join().expect("Cpu execution failed")?;
    Ok(result)
}

fn main() -> Result<()> {
    let prog = cpu::parse_input("resources/day13-input.txt")?;
    let tiles = solve1(&prog)?;
    // print_tiles(&tiles)?;
    println!("part 1: {}", tiles.values().filter(|t| *t == &'▢').count());
    Ok(())
}
