use anyhow::Result;
use cpu::Cpu;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::thread;
mod cpu;
mod util;

fn parse_input(path: &str) -> Result<Vec<i128>> {
    let s = std::fs::read_to_string(path)?;
    let result: std::result::Result<Vec<i128>, ParseIntError> =
        s.trim_end().split(',').map(|s| s.parse::<i128>()).collect();

    Ok(result?)
}

fn print_board(map: &HashMap<(isize, isize), char>) -> Result<()> {
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
            print!("{}", &ch.to_string());
        }
        println!();
    }

    Ok(())
}

fn build_map(prog: Vec<i128>) -> Result<HashMap<(isize, isize), char>> {
    let (mut cpu, _, rx) = Cpu::new(&prog);
    thread::spawn(move || cpu.execute());

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

fn find_origin(map: &HashMap<(isize, isize), char>) -> Result<((isize, isize), char)> {
    let origin = map.iter().find(|(_, ch)| match ch {
        '^' | 'v' | '<' | '>' => true,
        _ => false,
    });

    if origin.is_some() {
        return Ok((*origin.unwrap().0, *origin.unwrap().1));
    } else {
        anyhow::bail!("origin not found")
    }
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

fn forward(
    map: &HashMap<(isize, isize), char>,
    xy: (isize, isize),
    direction: char,
) -> ((isize, isize), isize) {
    let mut steps = 1;
    let mut x = xy.0;
    let mut y = xy.1;

    match direction {
        '^' => {
            while let Some('#') = map.get(&(xy.0, xy.1 - steps)) {
                steps += 1;
                y -= 1
            }
        }
        '<' => {
            while let Some('#') = map.get(&(xy.0 - steps, xy.1)) {
                steps += 1;
                x -= 1
            }
        }
        '>' => {
            while let Some('#') = map.get(&(xy.0 + steps, xy.1)) {
                steps += 1;
                x += 1
            }
        }
        'v' => {
            while let Some('#') = map.get(&(xy.0, xy.1 + steps)) {
                steps += 1;
                y += 1
            }
        }
        _ => {}
    }

    ((x, y), steps - 1)
}

fn turn(
    map: &HashMap<(isize, isize), char>,
    xy: (isize, isize),
    direction: char,
    l_or_r: char,
) -> Option<char> {
    let mut result = None;

    match (direction, l_or_r) {
        ('^', 'L') | ('v', 'R') => {
            if let Some('#') = map.get(&(xy.0 - 1, xy.1)) {
                result = Some('<')
            }
        }
        ('<', 'L') | ('>', 'R') => {
            if let Some('#') = map.get(&(xy.0, xy.1 + 1)) {
                result = Some('v')
            }
        }
        ('>', 'L') | ('<', 'R') => {
            if let Some('#') = map.get(&(xy.0, xy.1 - 1)) {
                result = Some('^')
            }
        }
        ('v', 'L') | ('^', 'R') => {
            if let Some('#') = map.get(&(xy.0 + 1, xy.1)) {
                result = Some('>')
            }
        }
        _ => {}
    }

    result
}

fn build_path(
    map: &HashMap<(isize, isize), char>,
    origin: (isize, isize),
    direction: char,
) -> String {
    let mut path: Vec<String> = vec![];
    let mut curpos = origin;
    let mut curdir = direction;

    loop {
        let (pos, steps) = forward(map, curpos, curdir);
        curpos = pos;

        if steps > 0 {
            path.push(steps.to_string());
        } else {
            match (
                turn(map, curpos, curdir, 'L'),
                turn(map, curpos, curdir, 'R'),
            ) {
                (None, None) => break,
                (Some(dir), _) => {
                    path.push("L".to_string());
                    curdir = dir
                }
                (None, Some(dir)) => {
                    path.push("R".to_string());
                    curdir = dir
                }
            }
        }
    }
    let path = path
        .iter()
        .fold("".to_string(), |acc, item| format!("{},{}", acc, item));
    path.trim_start_matches(",").to_string()
}

fn solve2(mut prog: Vec<i128>) -> Result<i128> {
    #[allow(unused_assignments)]
    let mut result = 0;

    prog[0] = 2;

    // manually founded these from path
    let a = "L,12,R,4,R,4";
    let b = "R,12,R,4,L,12";
    let c = "R,12,R,4,L,6,L,8,L,8";

    let main = "A,B,B,C,C,A,A,B,B,C";
    let inputs = vec![main, a, b, c, "n"];

    let (mut cpu, tx, rx) = Cpu::new(&prog);
    thread::spawn(move || cpu.execute());

    for input in inputs {
        loop {
            match rx.recv() {
                Ok(58) | Ok(63) => break,
                Ok(_) => (),
                Err(e) => anyhow::bail!("recv error: {:?}", e),
            }
        }
        for byte in input.bytes() {
            tx.send(byte as i128)?;
        }
        tx.send(10)?;
    }

    loop {
        match rx.recv() {
            Ok(x) if x > 255 => {
                result = x;
                break;
            }
            _ => (),
        }
    }

    Ok(result)
}

fn main() -> Result<()> {
    let prog = parse_input("resources/day17-input.txt")?;
    let prog2 = prog.clone();
    let map = build_map(prog)?;

    print_board(&map)?;
    println!("part 1: {}", solve1(&map)?);

    let (origin, direction) = find_origin(&map)?;
    let path = build_path(&map, origin, direction);

    println!("path: {}", path);
    println!("part 2: {}", solve2(prog2)?);

    Ok(())
}
