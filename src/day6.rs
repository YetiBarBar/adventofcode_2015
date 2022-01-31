use std::str::FromStr;

use adventofcode_2015::Matrix2D;
use adventofcode_tooling::{read_lines_to_vec_t, AocError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LightCmd {
    On,
    Off,
    Toggle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LightStatus {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LightCommand {
    cmd: LightCmd,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

struct Delta(usize, usize);

impl FromStr for Delta {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<_> = s.trim().split(',').collect();
        if s.len() != 2 {
            return Err(AocError::ParsingError);
        }

        let x = s[0].parse().map_err(|_| AocError::ParsingError)?;
        let y = s[1].parse().map_err(|_| AocError::ParsingError)?;

        Ok(Self(x, y))
    }
}

impl FromStr for LightCommand {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, s) = if s.starts_with("turn on ") {
            (LightCmd::On, s.trim_start_matches("turn on "))
        } else if s.starts_with("turn off ") {
            (LightCmd::Off, s.trim_start_matches("turn off "))
        } else if s.starts_with("toggle ") {
            (LightCmd::Toggle, s.trim_start_matches("toggle "))
        } else {
            return Err(AocError::ParsingError);
        };

        let splits: Vec<_> = s.split(' ').collect();

        if splits.len() != 3 {
            return Err(AocError::ParsingError);
        }

        let Delta(x_min, y_min) = splits[0].parse::<Delta>()?;
        let Delta(x_max, y_max) = splits[2].parse::<Delta>()?;

        Ok(Self {
            cmd,
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
fn part_1(data: &[LightCommand]) -> usize {
    let mut matrix: Matrix2D<LightStatus> = Matrix2D::new(1_000, 1_000);
    matrix.values = vec![LightStatus::Off; 1_000_000];
    for cmd in data {
        for idx_x in cmd.x_min..=cmd.x_max {
            for idx_y in cmd.y_min..=cmd.y_max {
                match cmd.cmd {
                    LightCmd::On => matrix.set_x_y(idx_x, idx_y, LightStatus::On),
                    LightCmd::Off => matrix.set_x_y(idx_x, idx_y, LightStatus::Off),
                    LightCmd::Toggle => {
                        if matrix.get_x_y(idx_x, idx_y) == LightStatus::On {
                            matrix.set_x_y(idx_x, idx_y, LightStatus::Off);
                        } else {
                            matrix.set_x_y(idx_x, idx_y, LightStatus::On);
                        }
                    }
                }
            }
        }
    }

    matrix
        .values
        .iter()
        .filter(|&v| *v == LightStatus::On)
        .count()
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
fn part_2(data: &[LightCommand]) -> usize {
    let mut matrix: Matrix2D<usize> = Matrix2D::new(1_000, 1_000);
    matrix.values = vec![0_usize; 1_000_000];
    for cmd in data {
        for idx_x in cmd.x_min..=cmd.x_max {
            for idx_y in cmd.y_min..=cmd.y_max {
                match cmd.cmd {
                    LightCmd::On => {
                        matrix.set_x_y(idx_x, idx_y, matrix.get_x_y(idx_x, idx_y) + 1);
                    }
                    LightCmd::Off => {
                        matrix.set_x_y(
                            idx_x,
                            idx_y,
                            matrix.get_x_y(idx_x, idx_y).saturating_sub(1),
                        );
                    }
                    LightCmd::Toggle => {
                        matrix.set_x_y(idx_x, idx_y, matrix.get_x_y(idx_x, idx_y) + 2);
                    }
                }
            }
        }
    }

    matrix.values.iter().sum()
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();

    let values: Vec<LightCommand> = read_lines_to_vec_t(r#"day_2015_6.data"#);

    println!("Part 1: {:?}", part_1(&values));
    println!("Part 2: {:?}", part_2(&values));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}
