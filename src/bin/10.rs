use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<i32> {
    let cpu = Cpu::new(input);
    let sum = cpu
        .skip(19)
        .step_by(40)
        .map(|s| s.cycle_num * s.reg_x)
        .sum();
    Some(sum)
}

pub fn part_two(_input: &str) -> Option<i32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_instruction(input: &str) -> Instruction {
    if input == "noop" {
        return Instruction::Noop;
    } else if input.starts_with("addx ") {
        if let Some((_, digits)) = input.split_once(' ') {
            let amount: i32 = digits.parse().unwrap();
            return Instruction::Addx(amount);
        }
    }
    panic!(
        "Something went wrong! Don't understand instruction <{}>",
        input
    )
}

fn parse_instruction_stream(input: &str) -> VecDeque<Instruction> {
    input.lines().map(parse_instruction).collect()
}

struct Cpu {
    instruction_stream: VecDeque<Instruction>,
    cycle_num: i32,
    reg_x: i32,
    current_operation: Instruction,
    wait_cycles: u8,
    done: bool,
}

#[derive(Debug)]
struct CpuState {
    cycle_num: i32,
    reg_x: i32,
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

#[cfg(test)]
mod tests {
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
        assert_eq!(part_two(&input), None);
    }
}
