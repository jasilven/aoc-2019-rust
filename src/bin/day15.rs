use anyhow::Result;
use rustbox::{Color, RustBox};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
mod cpu;
mod util;
use cpu::Cpu;

fn print_board(
    map: &HashMap<(isize, isize), char>,
    center: &(isize, isize),
    rb: &RustBox,
) -> Result<()> {
    rb.clear();

    let min_x = center.0 - rb.width() as isize / 2;
    let max_x = min_x + rb.width() as isize - 1;
    let min_y = center.1 - rb.height() as isize / 2;
    let max_y = min_y + rb.height() as isize - 1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let ch = map.get(&(x, y)).unwrap_or(&' ');
            rb.print_char(
                (x - min_x) as usize,
                (y - min_y) as usize,
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

    Ok(())
}

fn solve(path: &str, rb: &RustBox) -> Result<HashMap<(isize, isize), char>> {
    let prog = cpu::parse_input(path)?;
    let (tx, receiver): (Sender<i128>, Receiver<i128>) = mpsc::channel();
    let (sender, rx): (Sender<i128>, Receiver<i128>) = mpsc::channel();

    thread::spawn(move || {
        let _cpu = Cpu::new_with_send_recv(&prog, sender, receiver).execute();
    });

    #[allow(unused_assignments)]
    let (mut curpos, mut center, mut newpos) =
        ((0isize, 0isize), (0isize, 0isize), (0isize, 0isize));
    #[allow(unused_assignments)]
    let mut curdir = 0;

    let mut map = HashMap::new();
    map.insert(curpos, 'S');

    let mut backtrack: VecDeque<isize> = VecDeque::new();

    loop {
        let mut backtracking = false;
        let north = (curpos.0, curpos.1 + 1);
        let south = (curpos.0, curpos.1 - 1);
        let west = (curpos.0 + 1, curpos.1);
        let east = (curpos.0 - 1, curpos.1);

        curdir = match (
            map.get(&north),
            map.get(&south),
            map.get(&west),
            map.get(&east),
        ) {
            (None, _, _, _) => 1,
            (_, None, _, _) => 2,
            (_, _, None, _) => 3,
            (_, _, _, None) => 4,
            _ => {
                backtracking = true;
                match backtrack.pop_front() {
                    Some(dir) => dir,
                    None => break,
                }
            }
        };

        newpos = match curdir {
            1 => north,
            2 => south,
            3 => west,
            4 => east,
            _ => anyhow::bail!("invalid direction: {}", curdir),
        };

        tx.send(curdir as i128)?;

        match rx.recv() {
            Ok(0) => {
                map.insert(newpos, '#');
            }
            Ok(x) if (x == 1) || (x == 2) => {
                match x {
                    1 => {
                        map.insert(newpos, 'D');
                        map.insert(
                            curpos,
                            match map.get(&curpos) {
                                Some('D') | None => '.',
                                Some(ch) => *ch,
                            },
                        );
                    }
                    _ => {
                        map.insert(newpos, 'O');
                        map.insert(curpos, '.');
                    }
                }
                if !backtracking {
                    let backdir = match curdir {
                        1 => 2,
                        2 => 1,
                        3 => 4,
                        4 => 3,
                        _ => anyhow::bail!("invalid curdir: {} ", &curdir),
                    };
                    backtrack.push_front(backdir);
                }
                curpos = newpos;
            }
            Ok(x) => anyhow::bail!("invalid response {}", x),
            Err(e) => anyhow::bail!("Error: {:?}", e),
        }

        if (center.0 - curpos.0).abs() >= rb.width() as isize / 2 {
            center.0 = curpos.0;
        }
        if (center.1 - curpos.1).abs() >= rb.height() as isize / 2 {
            center.1 = curpos.1;
        }

        print_board(&map, &center, rb)?;
    }

    print_board(&map, &center, rb)?;
    rb.print(
        0,
        0,
        rustbox::RB_BOLD,
        Color::Red,
        Color::Default,
        &format!(
            "Distance from (D)roid to (O)xygen is {} steps.",
            bfs('D', 'O', &map)?
        ),
    );
    rb.print(
        0,
        1,
        rustbox::RB_BOLD,
        Color::Red,
        Color::Default,
        &format!(
            "It takes {} minutes to fill all locations with oxygen. Press any key to exit.",
            solve2(&map)?
        ),
    );
    rb.present();

    loop {
        match rb.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(_)) => break,
            Ok(_) => {}
            Err(e) => anyhow::bail!(e),
        }
    }
    Ok(map)
}

fn bfs(origin: char, target: char, map: &HashMap<(isize, isize), char>) -> Result<usize> {
    let mut result = 0;
    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    let mut points_to_visit: VecDeque<((isize, isize), isize)> = VecDeque::new();
    let (start, _) = map
        .iter()
        .find(|(_, val)| *val == &origin)
        .ok_or_else(|| anyhow::anyhow!("origin not found"))?;

    points_to_visit.push_back((*start, 0));

    while !points_to_visit.is_empty() {
        let (curpos, steps) = points_to_visit.pop_front().unwrap();
        seen.insert(curpos);

        if map.get(&curpos) == Some(&target) {
            result = steps as usize;
            break;
        }

        for neighbour in [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(dx, dy)| (curpos.0 + dx, curpos.1 + dy))
        {
            match map.get(&neighbour) {
                None | Some('#') => continue,
                _ => {
                    if !seen.contains(&neighbour) {
                        points_to_visit.push_back((neighbour, steps + 1));
                    }
                }
            }
        }
    }

    Ok(result)
}

fn solve2(map: &HashMap<(isize, isize), char>) -> Result<usize> {
    let mut result = 0usize;
    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    let mut points_to_visit: VecDeque<((isize, isize), isize)> = VecDeque::new();
    let (start, _) = map
        .iter()
        .find(|(_, val)| *val == &'O')
        .ok_or_else(|| anyhow::anyhow!("oxygen not found"))?;

    let mut minute = 0;

    points_to_visit.push_back((*start, 0));

    while !points_to_visit.is_empty() {
        let (curpos, steps) = points_to_visit.pop_front().unwrap();

        if steps != minute {
            result += 1;
            minute = steps;
        }

        seen.insert(curpos);

        for neighbour in [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(dx, dy)| (curpos.0 + dx, curpos.1 + dy))
        {
            match map.get(&neighbour) {
                None | Some('#') => continue,
                _ => {
                    if !seen.contains(&neighbour) {
                        points_to_visit.push_back((neighbour, steps + 1));
                    }
                }
            }
        }
    }

    Ok(result)
}

fn main() -> Result<()> {
    let rb = RustBox::init(Default::default())?;
    let _map = solve("resources/day15-input.txt", &rb)?;

    Ok(())
}
