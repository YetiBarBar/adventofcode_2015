use nom::bytes::complete::tag;
use nom::sequence::{preceded, separated_pair};
use nom::{
    branch::alt,
    character::{is_alphabetic, is_digit},
    combinator::{map, map_res},
    IResult,
};
use std::collections::HashMap;
use std::ops::Not;

#[derive(Debug, Clone)]
enum Value {
    Reg(String),
    Int(u16),
}

#[derive(Debug, Clone)]
enum Gate {
    Not(Value),
    Or(Value, Value),
    And(Value, Value),
    Rshift(Value, Value),
    Lshift(Value, Value),
    Fixed(Value),
}

impl Value {
    fn evaluate(&self, hmap: &HashMap<String, Gate>, cache: &mut HashMap<String, u16>) -> u16 {
        match self {
            Value::Reg(reg) => {
                if let Some(v) = cache.get(reg) {
                    *v
                } else {
                    let value = hmap.get(reg).unwrap().evaluate(hmap, cache);
                    cache.insert(reg.to_string(), value);
                    value
                }
            }
            Value::Int(v) => *v,
        }
    }
}

impl Gate {
    fn evaluate(&self, hmap: &HashMap<String, Gate>, cache: &mut HashMap<String, u16>) -> u16 {
        match self {
            Gate::Not(v1) => v1.evaluate(hmap, cache).not(),
            Gate::Or(v1, v2) => v1.evaluate(hmap, cache) | v2.evaluate(hmap, cache),
            Gate::And(v1, v2) => v1.evaluate(hmap, cache) & v2.evaluate(hmap, cache),
            Gate::Rshift(v1, v2) => v1.evaluate(hmap, cache) >> v2.evaluate(hmap, cache),
            Gate::Lshift(v1, v2) => v1.evaluate(hmap, cache) << v2.evaluate(hmap, cache),
            Gate::Fixed(v) => v.evaluate(hmap, cache),
        }
    }
}

fn parse_value(data: &[u8]) -> IResult<&[u8], Value> {
    alt((
        map_res(nom::bytes::complete::take_while1(is_digit), |bytes| {
            String::from_utf8_lossy(bytes)
                .parse::<u16>()
                .map(Value::Int)
        }),
        map(nom::bytes::complete::take_while1(is_alphabetic), |bytes| {
            Value::Reg(String::from_utf8_lossy(bytes).to_string())
        }),
    ))(data)
}

fn parse_not(data: &[u8]) -> IResult<&[u8], Gate> {
    map(preceded(tag(b"NOT "), parse_value), Gate::Not)(data)
}

fn parse_or(data: &[u8]) -> IResult<&[u8], Gate> {
    map(
        separated_pair(parse_value, tag(" OR "), parse_value),
        |(v1, v2)| Gate::Or(v1, v2),
    )(data)
}

fn parse_and(data: &[u8]) -> IResult<&[u8], Gate> {
    map(
        separated_pair(parse_value, tag(" AND "), parse_value),
        |(v1, v2)| Gate::And(v1, v2),
    )(data)
}
fn parse_lshift(data: &[u8]) -> IResult<&[u8], Gate> {
    map(
        separated_pair(parse_value, tag(" LSHIFT "), parse_value),
        |(v1, v2)| Gate::Lshift(v1, v2),
    )(data)
}
fn parse_rshift(data: &[u8]) -> IResult<&[u8], Gate> {
    map(
        separated_pair(parse_value, tag(" RSHIFT "), parse_value),
        |(v1, v2)| Gate::Rshift(v1, v2),
    )(data)
}
fn parse_gate(data: &[u8]) -> IResult<&[u8], Gate> {
    alt((
        parse_and,
        parse_or,
        parse_not,
        parse_lshift,
        parse_rshift,
        parse_fixed,
    ))(data)
}

fn parse_fixed(data: &[u8]) -> IResult<&[u8], Gate> {
    map(parse_value, Gate::Fixed)(data)
}

fn parse_rule(data: &[u8]) -> IResult<&[u8], (Gate, String)> {
    separated_pair(
        parse_gate,
        tag(" -> "),
        map(nom::bytes::complete::take_while1(is_alphabetic), |bytes| {
            String::from_utf8_lossy(bytes).to_string()
        }),
    )(data)
}

fn main() {
    let input = include_str!("../data/day_2015_7.data");

    let mut hmap: HashMap<String, Gate> = input
        .lines()
        .map(|line| parse_rule(line.as_bytes()).unwrap())
        .map(|(_, (gate, reg))| (reg, gate))
        .collect();

    let mut cache = HashMap::new();

    println!(
        "Part 1: {}",
        hmap.get(&"a".to_string())
            .unwrap()
            .evaluate(&hmap, &mut cache)
    );

    let new_b = Gate::Fixed(Value::Int(3176));
    let mut cache2 = HashMap::new();
    hmap.insert("b".to_string(), new_b);

    println!(
        "Part 2: {}",
        hmap.get(&"a".to_string())
            .unwrap()
            .evaluate(&hmap, &mut cache2)
    );
}
