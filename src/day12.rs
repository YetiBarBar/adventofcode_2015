fn main() {
    let input = include_str!("../data/day12_json_pp.data");

    let valid_char: String = input
        .chars()
        .filter(|c| c.is_digit(10) || c == &'-' || c == &'\n')
        .collect();

    print!("{}", valid_char);
    let res1 = valid_char
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let cleaned: String = line
                .chars()
                .filter(|c| c.is_digit(10) || c == &'-')
                .collect();
            println!("{}", cleaned);
            cleaned.parse::<isize>().ok()
        })
        .sum::<isize>();

    println!("Part 1: {}", res1);
}
