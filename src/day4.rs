use adventofcode_tooling::AocError;

#[must_use]
pub fn process(data: &str, goal: &str, start: usize) -> usize {
    for idx in start.. {
        let data = format!("{}{}", data, idx);
        let hashed = format!("{:?}", md5::compute(data.as_bytes()));
        if hashed.starts_with(goal) {
            return idx;
        }
    }
    unreachable!()
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
#[must_use]
pub fn part_1(data: &str) -> usize {
    process(data, "00000", 1)
}

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
#[must_use]
pub fn part_2(data: &str, start: usize) -> usize {
    process(data, "000000", start)
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();
    let p1 = part_1("bgvyzdsv");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", part_2("bgvyzdsv", p1));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_day4_step1() {
        assert_eq!(part_1("abcdef"), 609043);
    }
}
