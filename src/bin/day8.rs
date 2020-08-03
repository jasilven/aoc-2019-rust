use anyhow::Result;

fn parse_input(path: &str, w: usize, h: usize) -> Result<Vec<Vec<Vec<u32>>>> {
    let data = std::fs::read_to_string(path)?;

    let digits = data
        .trim_end()
        .chars()
        .map(|ch| {
            ch.to_digit(10)
                .ok_or_else(|| anyhow::anyhow!("digit parse error"))
        })
        .collect::<Result<Vec<u32>>>()?;

    let rows: Vec<Vec<u32>> = digits
        .chunks(w)
        .map(|ch| ch.iter().cloned().collect())
        .collect();

    let layers: Vec<Vec<Vec<u32>>> = rows.chunks(h).map(|chunk| chunk.to_vec()).collect();

    Ok(layers)
}

fn solve1(layers: &Vec<Vec<Vec<u32>>>) -> Result<usize> {
    let layer = layers
        .iter()
        .min_by(|l1, l2| {
            l1.iter()
                .flatten()
                .filter(|i| **i == 0u32)
                .count()
                .cmp(&l2.iter().flatten().filter(|i| **i == 0u32).count())
        })
        .ok_or_else(|| anyhow::anyhow!("unable to find target layer"))?;
    let one_count = layer.iter().flatten().filter(|i| **i == 1).count();
    let two_count = layer.iter().flatten().filter(|i| **i == 2).count();

    Ok(one_count * two_count)
}

fn solve2(layers: &Vec<Vec<Vec<u32>>>, w: usize, h: usize) -> Result<()> {
    for y in 0..h {
        let mut line = String::from("");
        for x in 0..w {
            for layer in layers {
                match layer[y][x] {
                    0 => {
                        line.push(' ');
                        break;
                    }
                    1 => {
                        line.push('#');
                        break;
                    }
                    _ => (),
                }
            }
        }
       println!("{}", line);
    }

    Ok(())
}

fn main() -> Result<()> {
    let (w, h) = (25, 6);
    let layers = parse_input("resources/day8-input.txt", w, h).unwrap();

    println!("part 1: {}", solve1(&layers)?);
    println!("part 2: ");
    solve2(&layers, w, h)?;

    // ###   ##  #  # #     ##
    // #  # #  # #  # #    #  #
    // #  # #    #  # #    #  #
    // ###  #    #  # #    ####
    // #    #  # #  # #    #  #
    // #     ##   ##  #### #  #

    Ok(())
}

#[cfg(test)]

mod day8_tests {
    use super::*;

    #[test]
    fn parsing() {
        let layers = parse_input("resources/day8-test.txt", 3, 2).unwrap();
        assert_eq!(
            vec![
                vec![vec![1u32, 2, 3], vec![4, 5, 6]],
                vec![vec![7u32, 8, 9], vec![0, 1, 2]]
            ],
            layers
        );
    }
}

