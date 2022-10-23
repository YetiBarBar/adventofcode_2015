use std::collections::{HashMap, HashSet};

fn main() {
    let input_poly: Vec<char> = "CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr".chars().collect();
    let input = include_str!("../data/day19.data");

    let world_map = extract_world_map(input);
    let medicine = split_medicine(&input_poly);
    let calibrated = calibrate(&medicine, &world_map);

    println!("Part 1: {}", calibrated.len());
}

fn calibrate(medicine: &[String], world_map: &HashMap<String, Vec<String>>) -> HashSet<String> {
    let mut hset = HashSet::<String>::new();
    for (position, molecule) in medicine.iter().enumerate() {
        let mut clone = medicine.to_owned();
        for new_molecule in world_map.get(molecule).unwrap_or(&vec![]) {
            clone[position] = new_molecule.to_string();
            hset.insert(clone.join(""));
        }
    }
    hset
}

fn split_medicine(input_poly: &[char]) -> Vec<String> {
    let medicine: Vec<String> = input_poly.windows(2).fold(Vec::new(), |mut acc, item| {
        if item[0].is_ascii_lowercase() {
            acc
        } else if item[1].is_ascii_lowercase() {
            acc.push(format!("{}{}", item[0], item[1]));
            acc
        } else {
            acc.push(item[0].to_string());
            acc
        }
    });
    medicine
}

fn extract_world_map(input: &str) -> HashMap<String, Vec<String>> {
    let mut world_map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines().take_while(|ln| !ln.is_empty()) {
        let (left, right) = line.split_once("=>").unwrap();
        let (left, right) = (left.trim().to_string(), right.trim().to_string());
        world_map.entry(left).or_default().push(right);
    }
    world_map
}
