use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(isize),
    Jie(char, isize),
    Jio(char, isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_ascii_whitespace().collect();

        let chr = parts[1].chars().next().unwrap();
        match parts.first() {
            Some(&"hlf") => Ok(Self::Hlf(chr)),
            Some(&"tpl") => Ok(Self::Tpl(chr)),
            Some(&"inc") => Ok(Self::Inc(chr)),
            Some(&"jmp") => {
                let offset: Result<isize, _> = if chr == '+' {
                    parts[1][1..].parse()
                } else {
                    parts[1].parse()
                };
                Ok(Self::Jmp(offset.unwrap()))
            }
            Some(&"jie") => {
                let offset: Result<isize, _> = if chr == '+' {
                    parts[2][1..].parse()
                } else {
                    parts[2].parse()
                };
                Ok(Self::Jie(chr, offset.unwrap()))
            }
            Some(&"jio") => {
                let offset: Result<isize, _> = if chr == '+' {
                    parts[2][1..].parse()
                } else {
                    parts[2].parse()
                };
                Ok(Self::Jio(chr, offset.unwrap()))
            }
            _ => {
                println!("{}", parts[0]);
                unreachable!()
            }
        }
    }
}
#[derive(Debug)]
struct Cpu {
    register_a: usize,
    register_b: usize,
    eip: isize,
}

impl Cpu {
    fn new() -> Self {
        Self {
            register_a: 0,
            register_b: 0,
            eip: 0,
        }
    }

    fn new_step2() -> Self {
        Self {
            register_a: 1,
            register_b: 0,
            eip: 0,
        }
    }

    fn hlf(&mut self, register: char) {
        match register {
            'a' => self.register_a /= 2,
            'b' => self.register_b /= 2,
            _ => unreachable!(),
        }
        self.eip += 1;
    }

    fn tpl(&mut self, register: char) {
        match register {
            'a' => self.register_a *= 3,
            'b' => self.register_b *= 3,
            _ => unreachable!(),
        }
        self.eip += 1;
    }

    fn inc(&mut self, register: char) {
        match register {
            'a' => self.register_a += 1,
            'b' => self.register_b += 1,
            _ => unreachable!(),
        }
        self.eip += 1;
    }

    fn jmp(&mut self, offset: isize) {
        self.eip += offset;
    }

    fn jie(&mut self, register: char, offset: isize) {
        let reg_val = match register {
            'a' => self.register_a,
            'b' => self.register_b,
            _ => unreachable!(),
        };
        if reg_val % 2 == 0 {
            self.eip += offset;
        } else {
            self.eip += 1;
        };
    }

    fn jio(&mut self, register: char, offset: isize) {
        let reg_val = match register {
            'a' => self.register_a,
            'b' => self.register_b,
            _ => unreachable!(),
        };
        if reg_val == 1 {
            self.eip += offset;
        } else {
            self.eip += 1;
        };
    }

    fn exec_instr(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Hlf(chr) => self.hlf(*chr),
            Instruction::Tpl(chr) => self.tpl(*chr),
            Instruction::Inc(chr) => self.inc(*chr),
            Instruction::Jmp(offset) => self.jmp(*offset),
            Instruction::Jie(chr, offset) => self.jie(*chr, *offset),
            Instruction::Jio(chr, offset) => self.jio(*chr, *offset),
        }
    }

    fn exec_prog(&mut self, prog: &[Instruction]) -> usize {
        let mut bmax = 0;
        while self.eip >= 0 && usize::try_from(self.eip).unwrap() < prog.len() {
            let instr = prog[usize::try_from(self.eip).unwrap()];
            self.exec_instr(&instr);
            bmax = bmax.max(self.register_b);
        }
        bmax
    }
}

fn main() {
    let input = include_str!("../data/day23.data");
    let prog: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::parse::<Instruction>)
        .map(Result::unwrap)
        .collect();

    let mut cpu = Cpu::new();
    println!("Part 1: {}", cpu.exec_prog(&prog));

    let mut cpu = Cpu::new_step2();
    println!("Part 2: {}", cpu.exec_prog(&prog));
}
