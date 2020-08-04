use anyhow::Result;
use async_std::sync::{channel, Receiver, Sender};
use async_std::task;
use itertools::Itertools;
mod cpu_async;
use cpu_async::Cpu;

fn create_amp_circuit(
    prog: &[isize],
    init_seq: &[isize],
) -> Result<(Vec<Cpu>, Sender<isize>, Receiver<isize>)> {
    let mut senders: Vec<Sender<isize>> = vec![];
    let mut recvers: Vec<Receiver<isize>> = vec![];
    let mut cpus: Vec<Cpu> = vec![];

    for _ in 0..=init_seq.len() {
        let (tx, rx) = channel::<isize>(1);
        senders.push(tx);
        recvers.push(rx);
    }

    for (sender, init) in senders.iter().zip(init_seq.iter()) {
        task::block_on(async { sender.send(*init).await });
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

async fn solve1(prog: &[isize]) -> Result<usize> {
    let mut result = 0usize;
    let perms = (0..=4).permutations(5);

    for seq in perms {
        let (cpus, sender, recver) = create_amp_circuit(prog, &seq)?;

        task::spawn(async move {
            sender.send(0).await;
        });

        let mut jhs = vec![];

        for mut cpu in cpus {
            let jh = task::spawn(async move { cpu.execute().await });
            jhs.push(jh);
        }

        for jh in jhs {
            jh.await?;
        }

        result = std::cmp::max(recver.recv().await? as usize, result);
    }

    Ok(result)
}

async fn solve2(prog: &[isize]) -> Result<usize> {
    let mut result = 0usize;
    let perms = (5..=9).permutations(5);

    for seq in perms {
        let (cpus, sender, recver) = create_amp_circuit(prog, &seq)?;
        let sender2 = sender.clone();

        task::spawn(async move {
            sender.send(0).await;
        });

        let mut jhs = vec![];

        for mut cpu in cpus {
            let jh = task::spawn(async move { cpu.execute().await });
            jhs.push(jh);
        }

        loop {
            match recver.recv().await {
                Ok(val) => {
                    result = std::cmp::max(val as usize, result);
                    sender2.send(val).await;
                }
                Err(_) => break,
            };
        }
        for jh in jhs {
            jh.await?;
        }
    }

    Ok(result)
}

#[async_std::main]
async fn main() -> Result<()> {
    let prog = cpu_async::parse_input("resources/day7-input.txt")?;
    println!("part 1: {}", solve1(&prog).await?);
    println!("part 2: {}", solve2(&prog).await?);

    Ok(())
}
