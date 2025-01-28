use std::collections::HashMap;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::AsChar;
use nom::{
    combinator::{map, map_res},
    IResult, Parser,
};

fn parse_string(data: &[u8]) -> IResult<&[u8], String> {
    map(
        nom::bytes::complete::take_while1(AsChar::is_alpha),
        |bytes| String::from_utf8_lossy(bytes).to_string(),
    )
    .parse(data)
}

fn parse_int(data: &[u8]) -> IResult<&[u8], usize> {
    map_res(
        nom::bytes::complete::take_while1(AsChar::is_dec_digit),
        |bytes| String::from_utf8_lossy(bytes).parse::<usize>(),
    )
    .parse(data)
}

fn parse_line(data: &str) -> ((String, String), usize) {
    separated_pair(
        separated_pair(parse_string, tag(" to "), parse_string),
        tag(" = "),
        parse_int,
    )
    .parse(data.as_bytes())
    .unwrap()
    .1
}

fn evaluate_permutation(
    permut: &[&String],
    world_map: &HashMap<String, HashMap<String, usize>>,
) -> usize {
    permut
        .windows(2)
        .map(|win| world_map.get(win[0]).unwrap().get(win[1]).unwrap())
        .sum()
}

fn main() {
    let input = include_str!("../data/day_2015_9.data");

    let mut world_map = std::collections::HashMap::new();

    for line in input.lines() {
        let ((one, two), dist) = parse_line(line);
        world_map
            .entry(one.clone())
            .or_insert_with(HashMap::new)
            .insert(two.clone(), dist);

        world_map
            .entry(two)
            .or_insert_with(HashMap::new)
            .insert(one, dist);
    }

    let permut: Vec<_> = world_map
        .keys()
        .permutations(world_map.keys().count())
        .collect();

    let (min, max) = permut
        .iter()
        .map(|permut| evaluate_permutation(permut, &world_map))
        .minmax()
        .into_option()
        .unwrap();

    println!("Part 1: {}", min);
    println!("Part 2: {}", max);
}
