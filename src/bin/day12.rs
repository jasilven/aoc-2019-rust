use anyhow::Result;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_input(path: &str) -> Result<Vec<Vec<isize>>> {
    let mut result = vec![];

    let f = File::open(path)?;

    let reader = BufReader::new(f);

    for line in reader.lines() {
        let mut moon = vec![];

        let line = line?;
        let line = line.trim_start_matches('<').trim_end_matches('>');
        let splits: Vec<&str> = line.split(", ").collect();
        for split in splits {
            moon.push(split[2..].parse::<isize>()?);
        }
        let mut velocities = vec![0, 0, 0];
        moon.append(&mut velocities);
        result.push(moon);
    }

    Ok(result)
}

fn apply_gravity(moons: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let mut result = vec![];
    for moon in &moons {
        let mut new_moon = moon.clone();
        for m in &moons {
            if moon == m {
                continue;
            } else {
                for i in 0..3 {
                    match true {
                        true if (moon[i] < m[i]) => new_moon[i + 3] += 1,
                        true if (moon[i] > m[i]) => new_moon[i + 3] -= 1,
                        _ => (),
                    }
                }
            }
        }
        result.push(new_moon);
    }

    result
}

fn apply_velocity(moons: &mut Vec<Vec<isize>>) {
    for moon in moons {
        for i in 0..3 {
            moon[i] += moon[i + 3];
        }
    }
}

fn total_energy(moons: &Vec<Vec<isize>>) -> usize {
    let mut result = 0;
    for moon in moons {
        let potential: isize = moon[0..3].iter().map(|i| i.abs()).sum();
        let kinetic: isize = moon[3..6].iter().map(|i| i.abs()).sum();
        result += potential * kinetic;
        // (moon[0].abs() + moon[1].abs() + moon[2].abs()) * (moon[3] + moon[4] + moon[5])
    }
    result as usize
}

fn print_moons(moons: &Vec<Vec<isize>>, steps: usize) {
    println!("After {} steps:", steps);
    for moon in moons.iter() {
        println!(
            "pos=<x={:>3}, y={:>3}, z={:>3}>, vel=<x={:>3}, y={:>3}, z={:>3}>",
            moon[0], moon[1], moon[2], moon[3], moon[4], moon[5]
        );
    }
}

fn step(moons: Vec<Vec<isize>>, steps: usize) -> Vec<Vec<isize>> {
    let mut new_moons = moons;
    for _n in 0..steps {
        new_moons = apply_gravity(new_moons);

        apply_velocity(&mut new_moons);

        // print_moons(&new_moons, n + 1);
    }
    new_moons
}

fn solve1(moons: Vec<Vec<isize>>) -> usize {
    let moons = step(moons, 1000);
    total_energy(&moons)
}

fn solve2(moons: Vec<Vec<isize>>) -> u128 {
    use num::integer::lcm;

    let mut zeros: [u128; 3] = [0, 0, 0];
    let mut moons = moons;
    let mut ticks = 0;
    loop {
        ticks += 1;
        moons = step(moons, 1);

        for i in 3..=5 {
            let mut vlocities = vec![];
            for moon in moons.iter() {
                vlocities.push(moon[i]);
            }
            if vlocities.iter().all(|v| v == &0) {
                zeros[i - 3] = ticks;
            }
        }
        if zeros.iter().all(|i| i != &0) {
            break;
        }
    }
    lcm(lcm(zeros[0], zeros[1]), zeros[2]) * 2
}

fn main() -> Result<()> {
    let moons = parse_input("resources/day12-input.txt")?;
    let moons2 = moons.clone();
    println!("part 1: {}", solve1(moons));
    println!("part 2: {}", solve2(moons2));

    Ok(())
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn test_part1() {
        let tests = vec![
            ("resources/day12-test.txt", 10, 179),
            ("resources/day12-test2.txt", 100, 1940),
        ];

        for test in tests {
            let mut moons = parse_input(test.0).unwrap();
            moons = step(moons, test.1);
            assert_eq!(test.2, total_energy(&moons));
        }
    }
}
