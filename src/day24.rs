use itertools::Itertools;

fn try_permutation(input: &[usize], size: usize, goal: usize) -> Option<usize> {
    input
        .iter()
        .permutations(size)
        .filter(|candidate| candidate.iter().copied().sum::<usize>() == goal)
        .map(|candidate| candidate.iter().copied().product::<usize>())
        .min()
}
fn main() {
    let input: Vec<usize> = include_str!("../data/day24.data")
        .lines()
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    println!("Part 1: {}", solve(&input, 3));
    println!("Part 2: {}", solve(&input, 4));
}

fn solve(input: &[usize], parts: usize) -> usize {
    let goal = input.iter().sum::<usize>() / parts;
    for idx in 0..input.len() {
        if let Some(value) = try_permutation(input, idx, goal) {
            return value;
        }
    }
    unreachable!()
}
