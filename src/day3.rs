use adventofcode_tooling::AocError;
use std::collections::HashMap;
use std::{fs::read_to_string, path::PathBuf};

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_1(data: &str) -> Result<usize, &'static str> {
    let hmap = data
        .chars()
        .fold(
            (
                vec![((0_isize, 0_isize), 1_usize)]
                    .iter()
                    .copied()
                    .collect::<HashMap<(isize, isize), usize>>(),
                0_isize,
                0_isize,
            ),
            |(mut hmap, mut x, mut y), ch| {
                match ch {
                    '^' => {
                        y += 1;
                        *hmap.entry((x, y)).or_insert(0) += 1;
                    }
                    'v' => {
                        y -= 1;
                        *hmap.entry((x, y)).or_insert(0) += 1;
                    }
                    '<' => {
                        x -= 1;
                        *hmap.entry((x, y)).or_insert(0) += 1;
                    }
                    '>' => {
                        x += 1;
                        *hmap.entry((x, y)).or_insert(0) += 1;
                    }
                    _ => (),
                }
                (hmap, x, y)
            },
        )
        .0;
    // println!("{}", hmap.iter().map(|(_, v)| v).sum::<usize>());
    Ok(hmap.len())
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_2(data: &str) -> Result<usize, &'static str> {
    let hmap = data
        .chars()
        .fold(
            (
                vec![((0_isize, 0_isize), 2_usize)]
                    .iter()
                    .copied()
                    .collect::<HashMap<(isize, isize), usize>>(),
                0_isize,
                0_isize,
                0_isize,
                0_isize,
                false,
            ),
            |(mut hmap, mut x, mut y, mut rx, mut ry, robot), ch| {
                match ch {
                    '^' => {
                        if robot {
                            ry += 1;
                            *hmap.entry((rx, ry)).or_insert(0) += 1;
                        } else {
                            y += 1;
                            *hmap.entry((x, y)).or_insert(0) += 1;
                        };
                    }
                    'v' => {
                        if robot {
                            ry -= 1;
                            *hmap.entry((rx, ry)).or_insert(0) += 1;
                        } else {
                            y -= 1;
                            *hmap.entry((x, y)).or_insert(0) += 1;
                        }
                    }
                    '<' => {
                        if robot {
                            rx -= 1;
                            *hmap.entry((rx, ry)).or_insert(0) += 1;
                        } else {
                            x -= 1;
                            *hmap.entry((x, y)).or_insert(0) += 1;
                        }
                    }
                    '>' => {
                        if robot {
                            rx += 1;
                            *hmap.entry((rx, ry)).or_insert(0) += 1;
                        } else {
                            x += 1;
                            *hmap.entry((x, y)).or_insert(0) += 1;
                        }
                    }
                    _ => (),
                }

                (hmap, x, y, rx, ry, !robot)
            },
        )
        .0;

    Ok(hmap.len())
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();
    let mut filepath: PathBuf = std::env::current_dir()?;
    filepath.push("data");
    filepath.push("day_2015_3.data");
    let values = read_to_string(filepath)?;
    println!("Part 1: {:?}", part_1(&values));
    println!("Part 2: {:?}", part_2(&values));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_day3_step1() {
        assert_eq!(part_1("^v^v^v^v^v"), Ok(2));
        assert_eq!(part_1("^>v<"), Ok(4));
        assert_eq!(part_1(">"), Ok(2));
    }

    #[test]
    fn test_day3_step2() {
        assert_eq!(part_2("^v^v^v^v^v"), Ok(11));
        assert_eq!(part_2("^>v<"), Ok(3));
        // assert_eq!(part_2(">"), Ok(2));
    }
}
