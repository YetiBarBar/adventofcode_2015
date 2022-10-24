use std::collections::HashMap;

#[must_use]
pub fn house_n(n: usize) -> usize {
    (1..=n).map(|x| if n % x == 0 { x * 10 } else { 0 }).sum()
}

pub fn main() {
    /*
    // Commenting part 1 as it took 22 minutes!
    for idx in 1.. {
        let val = house_n(idx);
        if idx % 100_000 == 0 {
            println!("{}", val);
        }
        if val >= 36_000_000 {
            println!("Part 1: {}", idx);
            break;
        }
    } */

    // Part 2 takes only 3s
    let mut hmap = HashMap::new();
    for idx in 1.. {
        for ctr in 0..50 {
            *hmap.entry(ctr * idx).or_default() += idx * 11;
        }
        let value: u64 = hmap.remove(&idx).unwrap();
        // println!("{} : {}", idx, value);
        if value >= 36_000_000 {
            println!("Part 2: {}", idx);
            break;
        }
    }
}
