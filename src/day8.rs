pub fn main() {
    let data = include_str!("../data/day8.data");
    // let data = include_str!("../data/test8.data");

    let res = data
        .lines()
        .map(|line| {
            let mut escaper = Escaper::new();

            println!(
                "{} + 2 - {}",
                line.len(),
                line.chars()
                    .map(|chr| escaper.push_char(chr))
                    .sum::<usize>()
            );
            line.len() + 2
                - line
                    .chars()
                    .map(|chr| escaper.push_char(chr))
                    .sum::<usize>()
        })
        .sum::<usize>();
    println!("Part 1: {}", res);

    let res2 = data
        .lines()
        .map(|line| encode_size(line) - line.len())
        .sum::<usize>();
    println!("Part 2: {}", res2);
}

fn encode_size(input: &str) -> usize {
    // Only '\' char and '"' increase size
    // + 2 for start and end ""
    2 + input
        .chars()
        .map(|chr| match chr {
            '\\' | '"' => 2,
            _ => 1,
        })
        .sum::<usize>()
}

enum EscapingStatus {
    NoEscape,
    Slash,
    SlashX,
    FirstHexDigit,
}

struct Escaper {
    status: EscapingStatus,
}

impl Escaper {
    fn new() -> Self {
        Self {
            status: EscapingStatus::NoEscape,
        }
    }

    fn push_char(&mut self, chr: char) -> usize {
        match self.status {
            EscapingStatus::NoEscape => {
                if chr == '\\' {
                    self.status = EscapingStatus::Slash;
                    0
                } else {
                    1
                }
            }
            EscapingStatus::Slash => {
                if chr == '\\' || chr == 'n' || chr == '"' {
                    self.status = EscapingStatus::NoEscape;
                    1
                } else if chr == 'x' {
                    self.status = EscapingStatus::SlashX;
                    0
                } else {
                    // 2
                    panic!("Unexpecting escaping!")
                }
            }
            EscapingStatus::SlashX => {
                if chr.is_ascii_hexdigit() {
                    self.status = EscapingStatus::FirstHexDigit;
                    0
                } else {
                    panic!("Expected first hex digit")
                }
            }
            EscapingStatus::FirstHexDigit => {
                if chr.is_ascii_hexdigit() {
                    self.status = EscapingStatus::NoEscape;
                    1
                } else {
                    panic!("Expected panic hex digit")
                }
            }
        }
    }
}
