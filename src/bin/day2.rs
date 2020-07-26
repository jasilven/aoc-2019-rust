use anyhow::Result;
use cpu::Cpu;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
mod cpu;

fn solve1(mut input: Vec<isize>, pos1: Option<isize>, pos2: Option<isize>) -> Result<isize> {
    input[1] = pos1.unwrap_or(input[1]);
    input[2] = pos2.unwrap_or(input[2]);

    let (tx, rx): (Sender<isize>, Receiver<isize>) = mpsc::channel();
    let mut cpu = Cpu::new(&input, rx, tx);
    cpu.execute()?;

    Ok(cpu.prog[0])
}

fn solve2(input: Vec<isize>) -> Result<isize> {
    for noun in 0..100 {
        for verb in 0..100 {
            let prog = input.to_vec();
            match solve1(prog, Some(noun), Some(verb)) {
                Ok(19690720) => return Ok(100 * noun + verb),
                _ => (),
            }
        }
    }
    anyhow::bail!("unable to find solution!")
}

fn main() -> Result<()> {
    let input = cpu::parse_input("resources/day2-input.txt")?;
    println!("part 1: {}", solve1(input.clone(), Some(12), Some(2))?);
    println!("part 2: {}", solve2(input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let input = cpu::parse_input("resources/day2-test.txt").unwrap();
        assert_eq!(3500, solve1(input, None, None).unwrap());
    }
}
