use anyhow::Result;
use itertools::Itertools;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
mod cpu;
use cpu::Cpu;

fn create_amp_circuit(
    prog: &[isize],
    init_seq: &[isize],
) -> Result<(Vec<Cpu>, Sender<isize>, Receiver<isize>)> {
    let mut senders: Vec<Sender<isize>> = vec![];
    let mut recvers: Vec<Receiver<isize>> = vec![];
    let mut cpus: Vec<Cpu> = vec![];

    for _ in 0..=init_seq.len() {
        let (tx, rx): (Sender<isize>, Receiver<isize>) = mpsc::channel();
        senders.push(tx);
        recvers.push(rx);
    }

    for (sender, init) in senders.iter().zip(init_seq.iter()) {
        sender.send(*init)?;
    }

    let sender = senders.remove(0);

    for _ in 0..init_seq.len() {
        let cpu = Cpu::new(prog, senders.remove(0), recvers.remove(0));
        cpus.push(cpu);
    }

    let recver = recvers.remove(0);

    anyhow::ensure!(senders.is_empty(), "expecting 0 senders");
    anyhow::ensure!(recvers.is_empty(), "expecting 0 receivers");

    Ok((cpus, sender, recver))
}

fn solve1(prog: &[isize]) -> Result<usize> {
    let mut result = 0usize;
    let perms = (0..=4).permutations(5);

    for seq in perms {
        let (cpus, sender, recver) = create_amp_circuit(prog, &seq)?;

        sender.send(0)?;

        let mut handles = vec![];

        for mut cpu in cpus {
            let th = thread::spawn(move || cpu.execute());
            handles.push(th);
        }
        result = std::cmp::max(recver.recv()? as usize, result);

        for th in handles {
            th.join().unwrap()?;
        }
    }

    Ok(result)
}

fn solve2(prog: &[isize]) -> Result<usize> {
    let mut result = 0usize;
    let perms = (5..=9).permutations(5);

    for seq in perms {
        let (cpus, sender, recver) = create_amp_circuit(prog, &seq)?;

        sender.send(0)?;

        let mut handles = vec![];

        for mut cpu in cpus {
            let th = thread::spawn(move || cpu.execute());
            handles.push(th);
        }

        let d = Duration::from_millis(50);
        loop {
            match recver.recv_timeout(d) {
                Ok(val) => {
                    result = std::cmp::max(val as usize, result);
                    if let Err(_) = sender.send(val) {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }

        for th in handles {
            th.join().unwrap()?;
        }
    }

    Ok(result)
}

fn main() -> Result<()> {
    let prog = cpu::parse_input("resources/day7-input.txt")?;
    println!("part 1: {}", solve1(&prog)?);
    println!("part 2: {}", solve2(&prog)?);
    Ok(())
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    #[test]
    fn test_programs() {
        let progs = vec![
            vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ],
            vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ],
            vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ],
        ];

        let init_seqs = vec![
            vec![4, 3, 2, 1, 0],
            vec![0, 1, 2, 3, 4],
            vec![1, 0, 4, 3, 2],
        ];

        let results = vec![43210, 54321, 65210];

        for n in 0..progs.len() {
            let (cpus, sender, recver) = create_amp_circuit(&progs[n], &init_seqs[n]).unwrap();

            sender.send(0).unwrap();

            let mut handles = vec![];

            for mut cpu in cpus {
                let th = thread::spawn(move || {
                    cpu.execute().unwrap();
                });
                handles.push(th);
            }
            assert_eq!(results[n], recver.recv().unwrap());

            for th in handles {
                th.join().unwrap();
            }
        }
    }
}
