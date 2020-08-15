mod cpu;
mod util;
use anyhow::Result;
use cpu::Cpu;
use rustbox::{Color, RustBox};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn parse_input(path: &str) -> Result<Vec<i128>> {
    let s = std::fs::read_to_string(path)?;
    let result: std::result::Result<Vec<i128>, ParseIntError> =
        s.trim_end().split(',').map(|s| s.parse::<i128>()).collect();

    Ok(result?)
}

fn print_board(map: &HashMap<(isize, isize), char>, rb: &RustBox) -> Result<()> {
    rb.clear();

    let (max_x, _) = map
        .keys()
        .max_by_key(|(x, _)| x)
        .ok_or_else(|| anyhow::anyhow!("max_x not found"))?;
    let (_, max_y) = map
        .keys()
        .max_by_key(|(_, y)| y)
        .ok_or_else(|| anyhow::anyhow!("max_y not found"))?;

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            let ch = map.get(&(x, y)).unwrap_or(&' ');
            rb.print_char(
                x as usize,
                y as usize,
                rustbox::RB_NORMAL,
                match map.get(&(x, y)) {
                    Some('S') | Some('D') | Some('O') => Color::Red,
                    _ => Color::Default,
                },
                Color::Default,
                *ch,
            );
        }
    }

    rb.present();
    thread::sleep(std::time::Duration::from_millis(7000));
    Ok(())
}

fn build_map(prog: Vec<i128>) -> Result<HashMap<(isize, isize), char>> {
    let (_tx, receiver): (Sender<i128>, Receiver<i128>) = channel();
    let (sender, rx): (Sender<i128>, Receiver<i128>) = channel();

    thread::spawn(move || {
        let _cpu = Cpu::new(&prog, sender, receiver).execute();
    });

    let mut map = HashMap::new();

    let (mut y, mut x) = (0, 0);

    loop {
        match rx.recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(35) => {
                map.insert((x, y), '#');
                x += 1
            }
            Ok(46) => {
                map.insert((x, y), '.');
                x += 1
            }
            Ok(94) => {
                map.insert((x, y), '^');
                x += 1
            }
            Ok(10) => {
                y += 1;
                x = 0
            }
            Ok(x) => anyhow::bail!("got invalid response: {}", x),
            Err(_) => break,
        }
    }

    Ok(map)
}

fn solve1(map: &HashMap<(isize, isize), char>) -> Result<isize> {
    let result = map
        .iter()
        .filter(|((x, y), ch)| {
            *ch == &'#'
                && util::neighbours(&(*x, *y))
                    .iter()
                    .all(|xy| map.get(xy).unwrap_or(&' ') == &'#')
        })
        .map(|((x, y), _)| (x * y) as isize)
        .sum();

    Ok(result)
}

fn main() -> Result<()> {
    let prog = parse_input("resources/day17-input.txt")?;
    // let rb = RustBox::init(Default::default())?;
    let map = build_map(prog)?;
    // print_board(&map, &rb)?;
    println!("part 1: {}", solve1(&map)?);

    Ok(())
}
