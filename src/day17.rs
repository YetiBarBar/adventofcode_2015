/*
50
44
11
49
42
46
18
32
26
40
21
7
18
43
10
47
36
24
22
40
*/
pub fn main() {
    let input = [
        50, 44, 11, 49, 42, 46, 18, 32, 26, 40, 21, 7, 18, 43, 10, 47, 36, 24, 22, 40,
    ];

    let raw: Vec<_> = (0..=2_isize.pow(20))
        .map(|v| format!("{:#32b}", v).chars().rev().collect::<Vec<char>>())
        .map(|v| {
            let s = v
                .iter()
                .chain(std::iter::repeat(&'0'))
                .zip(input)
                .map(|(chr, val)| if chr == &'1' { val } else { 0 })
                .sum::<usize>();
            (v, s)
        })
        .filter(|(_, sum)| sum == &150)
        .collect();

    println!("Part 1: {}", raw.len());

    let min = raw
        .iter()
        .map(|(v, _)| v.iter().filter(|chr| chr == &&'1').count())
        .min()
        .unwrap();
    let count = raw
        .iter()
        .filter(|(v, _)| v.iter().filter(|chr| chr == &&'1').count() == min)
        .count();
    println!("Part 2: {}", count);
}
