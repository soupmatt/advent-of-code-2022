use std::{collections::VecDeque, ops::RangeInclusive};

pub fn part_one(input: &str) -> Option<isize> {
    let cpu = Cpu::new(input);
    let sum = cpu
        .skip(19)
        .step_by(40)
        .map(|s| isize::try_from(s.cycle_num).unwrap() * s.reg_x)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<String> {
    let cpu = Cpu::new(input);
    let mut screen = Screen::new();
    cpu.take(6 * 40).for_each(|cs| screen.draw_pixel(cs));
    Some(screen.to_string())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(isize),
}

fn parse_instruction(input: &str) -> Instruction {
    if input == "noop" {
        return Instruction::Noop;
    } else if input.starts_with("addx ") {
        if let Some((_, digits)) = input.split_once(' ') {
            let amount: isize = digits.parse().unwrap();
            return Instruction::Addx(amount);
        }
    }
    panic!("Something went wrong! Don't understand instruction <{input}>")
}

fn parse_instruction_stream(input: &str) -> VecDeque<Instruction> {
    input.lines().map(parse_instruction).collect()
}

struct Cpu {
    instruction_stream: VecDeque<Instruction>,
    cycle_num: usize,
    reg_x: isize,
    current_operation: Instruction,
    wait_cycles: u8,
    done: bool,
}

#[derive(Debug)]
struct CpuState {
    cycle_num: usize,
    reg_x: isize,
}

impl Cpu {
    fn new(instruction_input: &str) -> Cpu {
        let instruction_stream = parse_instruction_stream(instruction_input);
        Cpu {
            instruction_stream,
            cycle_num: 0,
            reg_x: 1,
            current_operation: Instruction::Noop,
            wait_cycles: 0,
            done: false,
        }
    }

    fn to_cpu_state(&self) -> CpuState {
        CpuState {
            cycle_num: self.cycle_num,
            reg_x: self.reg_x,
        }
    }

    fn run_cycle(&mut self) -> bool {
        self.cycle_num += 1;
        if self.wait_cycles == 0 {
            self.process_instruction()
        } else {
            self.wait_cycles -= 1;
            true
        }
    }

    fn process_instruction(&mut self) -> bool {
        match self.current_operation {
            Instruction::Noop => (),
            Instruction::Addx(val) => self.reg_x += val,
        }
        if let Some(next_instr) = self.instruction_stream.pop_front() {
            self.current_operation = next_instr;
            if let Instruction::Addx(_) = self.current_operation {
                self.wait_cycles = 1
            }
            true
        } else {
            false
        }
    }
}

impl Iterator for Cpu {
    type Item = CpuState;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        self.done = !self.run_cycle();
        Some(self.to_cpu_state())
    }
}

#[derive(Debug)]
struct Screen([[bool; 40]; 6]);

impl Screen {
    fn new() -> Screen {
        Screen([[false; 40]; 6])
    }

    fn draw_pixel(&mut self, cs: CpuState) {
        let sprite = Screen::sprite_position(&cs);
        let position = (cs.cycle_num - 1) % 40;
        let lit = sprite.contains(&position);
        let row = (cs.cycle_num - 1) / 40;
        let col = position;
        self.0[row][col] = lit;
    }

    fn sprite_position(cs: &CpuState) -> RangeInclusive<usize> {
        let val = usize::try_from(cs.reg_x).unwrap_or(0usize) % 40;
        match val {
            0 => 0..=1,
            _ => val - 1..=val + 1,
        }
    }
}

impl ToString for Screen {
    fn to_string(&self) -> String {
        self.0
            .map(|line| {
                line.map(|b| if b { '#' } else { '.' })
                    .iter()
                    .collect::<String>()
            })
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(parse_instruction("noop"), Instruction::Noop);
        assert_eq!(parse_instruction("addx 5"), Instruction::Addx(5));
        assert_eq!(parse_instruction("addx 382"), Instruction::Addx(382));
        assert_eq!(parse_instruction("addx -17"), Instruction::Addx(-17));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let expected = indoc! { "
            ##..##..##..##..##..##..##..##..##..##..
            ###...###...###...###...###...###...###.
            ####....####....####....####....####....
            #####.....#####.....#####.....#####.....
            ######......######......######......####
            #######.......#######.......#######.....
            "
        };
        assert_eq!(part_two(&input), Some(expected.trim_end().to_string()));
    }
}
