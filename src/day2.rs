use std::str::FromStr;

use adventofcode_tooling::{read_lines_to_vec_t, AocError};

struct Gift(isize, isize, isize);

impl FromStr for Gift {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<isize>, _> = s.split('x').map(|s| s.trim().parse::<isize>()).collect();
        if let Ok(res) = res {
            if res.len() == 3 {
                return Ok(Self(res[0], res[1], res[2]));
            }
        }
        Err(AocError::ParsingError)
    }
}

impl Gift {
    fn area(&self) -> isize {
        let &Gift(l, w, h) = self;
        2 * l * w + 2 * w * h + 2 * h * l + (l * w).min(w * h).min(h * l)
    }

    fn ribbon(&self) -> isize {
        let &Gift(l, w, h) = self;
        let bow = l * w * h;
        let v: Vec<isize> = {
            let mut v = vec![l, w, h];
            v.sort_unstable();
            v.iter().take(2).copied().collect()
        };
        2 * v.iter().sum::<isize>() + bow
    }
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
fn part_1(data: &[Gift]) -> isize {
    data.iter().map(Gift::area).sum::<isize>()
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
fn part_2(data: &[Gift]) -> isize {
    data.iter().map(Gift::ribbon).sum::<isize>()
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();

    let values: Vec<Gift> = read_lines_to_vec_t(r#"day_2015_2.data"#);
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

    fn test_day2_step1() {
        let value = vec!["2x3x4".parse::<Gift>().unwrap()];
        assert_eq!(part_1(&value), 58);
    }

    #[test]
    fn test_day2_step2() {
        let value = vec!["2x3x4".parse::<Gift>().unwrap()];
        assert_eq!(part_2(&value), 34);
    }
}
