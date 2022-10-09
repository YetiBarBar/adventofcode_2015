/*
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
*/

use std::str::FromStr;

#[derive(Debug)]
pub struct Aunt {
    id: usize,
    values: std::collections::HashMap<String, usize>,
}

impl FromStr for Aunt {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(':').unwrap();
        let mut hmap = std::collections::HashMap::new();
        let id = left
            .split_once(' ')
            .unwrap()
            .1
            .trim()
            .parse::<usize>()
            .unwrap();
        for part in right.split(',') {
            let (key, value) = part.split_once(':').unwrap();
            let key = key.trim().to_string();
            let value = value.trim().parse().unwrap();
            hmap.insert(key, value);
        }
        Ok(Aunt { id, values: hmap })
    }
}

pub fn main() {
    let input = include_str!("../data/day16.data");
    let aunts: Vec<Aunt> = input.lines().map(str::parse).map(Result::unwrap).collect();
    let aunt: Vec<_> = aunts
        .iter()
        .filter(|aunt| {
            let val = aunt.values.get(&"children".to_string());
            val == Some(&3) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"cats".to_string());
            val == Some(&7) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"samoyeds".to_string());
            val == Some(&2) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"pomeranians".to_string());
            val == Some(&3) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"akitas".to_string());
            val == Some(&0) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"vizslas".to_string());
            val == Some(&0) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"goldfish".to_string());
            val == Some(&5) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"trees".to_string());
            val == Some(&3) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"cars".to_string());
            val == Some(&2) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"perfumes".to_string());
            val == Some(&1) || val == None
        })
        .collect();
    println!("Part 1: {}", aunt.first().unwrap().id);
    let aunt: Vec<_> = aunts
        .iter()
        .filter(|aunt| {
            let val = aunt.values.get(&"children".to_string());
            val == Some(&3) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"cats".to_string());
            val > Some(&7) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"samoyeds".to_string());
            val == Some(&2) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"pomeranians".to_string());
            val < Some(&3) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"akitas".to_string());
            val == Some(&0) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"vizslas".to_string());
            val == Some(&0) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"goldfish".to_string());
            val < Some(&5) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"trees".to_string());
            val > Some(&3) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"cars".to_string());
            val == Some(&2) || val == None
        })
        .filter(|aunt| {
            let val = aunt.values.get(&"perfumes".to_string());
            val == Some(&1) || val == None
        })
        .collect();
    println!("Part 2: {:?}", aunt);
}
