pub fn main() {
    let mut test = "1113222113".to_string();
    for idx in 0..50 {
        test = turn(&test);
        if idx == 39 {
            println!("Part 1: {}", test.len());
        }
    }
    println!("Part 2: {}", test.len());
}

fn turn(input: &str) -> String {
    let mut retainee: Option<char> = None;
    let mut count = 0;
    let mut acc = Vec::new();
    for chr in input.chars() {
        if retainee.is_some() {
            if retainee == Some(chr) {
                count += 1;
            } else {
                acc.push(format!("{}{}", count, retainee.unwrap()));
                retainee = Some(chr);
                count = 1;
            }
        } else {
            retainee = Some(chr);
            count = 1;
        }
    }
    acc.push(format!("{}{}", count, retainee.unwrap()));
    acc.join("")
}
