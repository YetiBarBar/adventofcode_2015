struct DiagonalFiller {
    last_x: usize,
    last_y: usize,
    // max_y: usize,
}

impl DiagonalFiller {
    fn new() -> Self {
        Self {
            last_x: 1,
            last_y: 1,
            // max_y: 1,
        }
    }
}

impl Iterator for DiagonalFiller {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let next = Some((self.last_x, self.last_y));
        if self.last_y == 1 {
            self.last_y = self.last_x + 1;
            self.last_x = 1;
        } else {
            self.last_x += 1;
            self.last_y -= 1;
        }
        next
    }
}

struct ModGen {
    multiplier: usize,
    modulo: usize,
    next_val: usize,
}

impl ModGen {
    fn new(multiplier: usize, modulo: usize, start: usize) -> Self {
        Self {
            multiplier,
            modulo,
            next_val: start,
        }
    }
}

impl Iterator for ModGen {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.next_val;
        self.next_val = self.next_val * self.multiplier % self.modulo;
        Some(val)
    }
}

fn main() {
    // Input:
    // Code at row 3010, column 3019.
    let positioner = DiagonalFiller::new();
    let modgen = ModGen::new(252_533, 33_554_393, 20_151_125);
    let res = positioner
        .zip(modgen)
        .find(|((x, y), _)| x == &3019 && y == &3010)
        .unwrap_or_default()
        .1;
    println!("Part 1: {:?}", res);
}
