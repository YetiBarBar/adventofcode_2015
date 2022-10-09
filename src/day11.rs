pub fn main() {
    let mut pass = PassGenerator::new("cqjxjnds");
    while !pass.is_valid() {
        pass.next_candidate();
    }
    println!("Part 1: {}", pass.current());
    pass.next_candidate();
    while !pass.is_valid() {
        pass.next_candidate();
    }
    println!("Part 2: {}", pass.current());
}

struct PassGenerator {
    value: Vec<char>,
}

impl PassGenerator {
    fn new(initial_value: &str) -> Self {
        Self {
            value: initial_value.chars().collect(),
        }
    }

    fn is_valid(&self) -> bool {
        self.has_one_straight_forward()
            && self.non_overlapping_pairs()
            && self.no_forbidden_letter()
    }

    fn current(&self) -> String {
        self.value.iter().collect()
    }

    fn next_candidate(&mut self) -> String {
        // Determine first "non-'z'" letter!
        if let Some((increment_position, _)) = self
            .value
            .iter()
            .enumerate()
            .rev()
            .find(|(_, chr)| chr != &&'z')
        {
            self.value[increment_position] = (self.value[increment_position] as u8 + 1) as char;
            for idx in (increment_position + 1)..=7 {
                self.value[idx] = 'a';
            }
            self.value.iter().collect()
        } else {
            self.value = vec!['a'; 8];
            self.value.iter().collect()
        }
    }

    fn has_one_straight_forward(&self) -> bool {
        self.value.windows(3).any(|win| {
            win[0] != 'y'
                && win[0] != 'z'
                && (win[0] as u8 + 1 == win[1] as u8)
                && (win[0] as u8 + 2 == win[2] as u8)
        })
    }

    fn no_forbidden_letter(&self) -> bool {
        self.value
            .iter()
            .all(|chr| chr != &'i' && chr != &'o' && chr != &'l')
    }

    fn non_overlapping_pairs(&self) -> bool {
        let mut first_pair = None;
        let mut last = self.value.first().unwrap();
        for chr in self.value.iter().skip(1) {
            if last == chr {
                if let Some(other_char) = first_pair {
                    if other_char != chr {
                        return true;
                    }
                } else {
                    first_pair = Some(chr);
                }
            }
            last = chr;
        }
        false
    }
}
