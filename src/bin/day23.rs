use anyhow::Result;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

mod cpu;
use cpu::Cpu;

fn solve1(prog: &[i128]) -> Result<i128> {
    let mut cpus: Vec<(Sender<i128>, Receiver<i128>)> = vec![];
    let mut handles = vec![];

    // prepare cpus with nics
    for n in 0..50 {
        let (mut cpu, tx, rx) = Cpu::new(&prog);
        tx.send(n)?;
        cpus.push((tx, rx));
        let handle = thread::spawn(move || -> Result<()> { cpu.execute() });
        handles.push(handle);
    }

    // send & receive
    let mut packets = HashMap::<usize, Vec<i128>>::new();
    loop {
        for (sender_index, (_tx, rx)) in cpus.iter().enumerate() {
            match rx.try_recv() {
                Ok(val) => {
                    if let Some(v) = packets.get_mut(&sender_index) {
                        v.push(val);
                        if v.len() == 3 {
                            let target = *v.get(0).unwrap() as usize;
                            let x = v.get(1).unwrap();
                            let y = v.get(2).unwrap();
                            if target == 255usize {
                                return Ok(*y);
                            }
                            let cpu = &cpus[target];
                            cpu.0.send(*x)?;
                            cpu.0.send(*y)?;
                            packets.remove(&sender_index);
                        }
                    } else {
                        packets.insert(sender_index, vec![val]);
                    }
                }
                _ => {}
            }
        }
        for (_cpu, (tx, _)) in cpus.iter().enumerate() {
            tx.send(-1).unwrap();
        }
    }
}

fn solve2(prog: &[i128]) -> Result<i128> {
    let mut cpus: Vec<(Sender<i128>, Receiver<i128>)> = vec![];
    let mut handles = vec![];

    // prepare cpus with nics
    for n in 0..50 {
        let (mut cpu, tx, rx) = Cpu::new(&prog);
        tx.send(n)?;
        cpus.push((tx, rx));
        let handle = thread::spawn(move || -> Result<()> { cpu.execute() });
        handles.push(handle);
    }

    let mut packets = HashMap::<usize, Vec<i128>>::new();
    let mut nat_x = 0;
    let mut nat_y = 0;
    let mut last_y = None;
    let mut send_nat = false;

    // send & receive
    loop {
        let mut idle = true;
        for (sender_index, (_tx, rx)) in cpus.iter().enumerate() {
            match rx.try_recv() {
                Ok(val) => {
                    idle = false;
                    if let Some(v) = packets.get_mut(&sender_index) {
                        v.push(val);
                        if v.len() == 3 {
                            let target = *v.get(0).unwrap() as usize;
                            let x = v.get(1).unwrap();
                            let y = v.get(2).unwrap();
                            if target == 255usize {
                                nat_x = *x;
                                nat_y = *y;
                                send_nat = true;
                                idle = true;
                            } else {
                                let cpu = &cpus[target];
                                cpu.0.send(*x)?;
                                cpu.0.send(*y)?;
                            }
                            packets.remove(&sender_index);
                        }
                    } else {
                        packets.insert(sender_index, vec![val]);
                    }
                }
                _ => {}
            }
        }

        for (_cpu, (tx, _)) in cpus.iter().enumerate() {
            tx.send(-1).unwrap();
        }

        if idle && packets.is_empty() && send_nat {
            cpus[0].0.send(nat_x)?;
            cpus[0].0.send(nat_y)?;
            send_nat = false;
            if last_y.is_some() && (last_y.unwrap() == nat_y) {
                return Ok(nat_y);
            } else {
                last_y = Some(nat_y);
            }
        }
    }
}

fn main() -> Result<()> {
    let prog = cpu::parse_input("resources/day23-input.txt")?;

    println!("part 1: {}", solve1(&prog)?);
    println!("part 2: {}", solve2(&prog)?);

    Ok(())
}
