use std::collections::HashMap;

use adventofcode_tooling::{read_lines, AocError};

/// Process data for a given step
///
/// # Errors
///
/// can't produce error
fn part_1(data: &[String]) -> usize {
    data.iter().filter(|&s| is_nice(s)).count()
}

fn part_2(data: &[String]) -> usize {
    data.iter().filter(|&s| is_nice2(s)).count()
}

fn is_nice(in_data: &str) -> bool {
    vowels(in_data) && twice_in_a_row(in_data) && forbidden_sequence(in_data)
}

fn is_nice2(in_data: &str) -> bool {
    bisequence(in_data) && repeat_separated(in_data)
}

fn vowels(in_data: &str) -> bool {
    in_data
        .chars()
        .filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
        .count()
        .ge(&3)
}

fn bisequence(in_data: &str) -> bool {
    let v = in_data.chars().collect::<Vec<char>>();
    let v: Vec<_> = v.windows(2).collect();
    let hmap = v
        .iter()
        .fold(HashMap::<String, usize>::new(), |mut acc, &value| {
            let key = format!("{}{}", value[0], value[1]);
            *acc.entry(key).or_insert(0) += 1;
            acc
        });

    // A non-overlapping sequence is found only if
    // bi-sequence is found at least 3 times
    // or 2 two-times but we have not the seq "aaa"
    hmap.values().any(|v| v.gt(&2))
        || hmap
            .iter()
            .filter(|&(_, val)| *val == 2)
            .map(|(key, _)| key.chars().next().unwrap())
            .any(|ch| !in_data.contains(&format!("{}{}{}", ch, ch, ch)))
}

fn repeat_separated(in_data: &str) -> bool {
    let v = in_data.chars().collect::<Vec<char>>();
    v.windows(3).any(|v| v[0] == v[2])
}

fn twice_in_a_row(in_data: &str) -> bool {
    let v = in_data.chars().collect::<Vec<char>>();
    v.windows(2).any(|v| v[0] == v[1])
}

fn forbidden_sequence(in_data: &str) -> bool {
    let v = in_data.chars().collect::<Vec<char>>();
    v.windows(2).all(|v| {
        !matches!(
            (v[0], v[1]),
            ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')
        )
    })
}

/// Process solutions for day 1
///
/// # Errors
///
/// May fail if input data cannot be read
pub fn main() -> Result<(), AocError> {
    let now = std::time::Instant::now();

    let values: Vec<_> = read_lines(r#"day_2015_5.data"#)?.collect::<Result<Vec<_>, _>>()?;
    println!("Part 1: {}", part_1(&values));
    println!("Part 2: {}", part_2(&values));
    let elapsed = now.elapsed();
    println!("Exec time: {} \u{b5}s", elapsed.as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_day5_step1() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_day5_step2() {
        assert!(is_nice2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice2("xxyxx"));
        assert!(!is_nice2("uurcxstgmygtbstg"));
        assert!(!is_nice2("ieodomkazucvgmuy"));
    }
}
