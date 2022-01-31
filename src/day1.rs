use std::{fs::read_to_string, path::PathBuf};

use adventofcode_tooling::AocError;

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_1(data: &str) -> Result<isize, &'static str> {
    Ok(data.chars().fold(0_isize, |acc, ch| match ch {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    }))
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
pub fn part_2(data: &str) -> Result<usize, &'static str> {
    let mut position = 0_isize;
    for (index, ch) in data.chars().enumerate() {
        position += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if position.lt(&0) {
            return Ok(index + 1);
        }
    }
    Err("Not going dowstairs")
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
    filepath.push("day_2015_1.data");
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

    fn test_day1_step1() {
        assert_eq!(part_1("(())"), Ok(0));
        assert_eq!(part_1("()()"), Ok(0));
        assert_eq!(part_1("((("), Ok(3));
    }

    #[test]
    fn test_day1_step2() {
        assert_eq!(part_2("()())"), Ok(5));
    }
}
