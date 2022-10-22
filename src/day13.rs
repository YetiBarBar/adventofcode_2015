use std::collections::HashMap;

use itertools::Itertools;

fn parse_line(data: &str) -> ((String, String), isize) {
    let splits: Vec<&str> = data.split_ascii_whitespace().collect();
    let a = splits[0].into();
    let mut b = splits[10].to_string();
    b.pop();
    (
        (a, b),
        if splits[2] == "lose" {
            -splits[3].parse::<isize>().unwrap()
        } else {
            splits[3].parse().unwrap()
        },
    )
}

fn evaluate_permutation(
    permut: &[&String],
    world_map: &HashMap<String, HashMap<String, isize>>,
) -> isize {
    let mut total = 0;
    let empty = HashMap::new();
    for idx in 0..permut.len() {
        let current = permut[idx];
        let next = permut[(idx + 1) % permut.len()];
        let previous = if idx == 0 {
            permut[permut.len() - 1]
        } else {
            permut[idx - 1]
        };
        let local = world_map.get(current).unwrap_or(&empty);
        total += local.get(next).unwrap_or(&0);
        total += local.get(previous).unwrap_or(&0);
    }
    total
}

fn main() {
    let input = include_str!("../data/day13.data");

    let mut world_map = std::collections::HashMap::new();

    for line in input.lines() {
        let ((one, two), dist) = parse_line(line);
        world_map
            .entry(one.clone())
            .or_insert_with(HashMap::new)
            .insert(two.clone(), dist);
    }

    let max = eval_joy(&world_map);

    println!("Part 1: {}", max);

    // Inserting me!
    world_map.insert("me".to_string(), HashMap::new());

    let max = eval_joy(&world_map);

    println!("Part 2: {}", max);
}

fn eval_joy(world_map: &HashMap<String, HashMap<String, isize>>) -> isize {
    world_map
        .keys()
        .permutations(world_map.keys().count())
        .map(|permut| evaluate_permutation(&permut, world_map))
        .max()
        .unwrap()
}
