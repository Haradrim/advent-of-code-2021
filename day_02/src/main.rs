use std::{error::Error, str::FromStr};

use utils::read_file;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("input.txt")?;

    println!("Part 1: {:?}", part_01(&input));

    println!("Part 2: {:?}", part_02(&input));

    Ok(())
}

fn part_01(instructions: &String) -> u32 {
    let mut sub = Submarine {
        horizontal_pos: 0,
        depth: 0,
        aim: 0,
    };

    instructions
        .lines()
        .filter_map(|line| line.parse().ok())
        .for_each(|instruction| sub.execute_instruction(instruction));

    return sub.horizontal_pos * sub.depth;
}

fn part_02(instructions: &String) -> u32 {
    let mut sub = Submarine {
        horizontal_pos: 0,
        depth: 0,
        aim: 0,
    };

    instructions
        .lines()
        .filter_map(|line| line.parse().ok())
        .for_each(|instruction| sub.execute_instruction_p2(instruction));

    return sub.horizontal_pos * sub.depth;
}

struct Submarine {
    horizontal_pos: u32,
    depth: u32,
    aim: u32,
}

impl Submarine {
    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(arg) => self.horizontal_pos += arg,
            Instruction::Down(arg) => self.depth += arg,
            Instruction::Up(arg) => self.depth -= arg,
        }
    }

    fn execute_instruction_p2(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(arg) => {
                self.horizontal_pos += arg;
                self.depth += arg * self.aim;
            }
            Instruction::Down(arg) => self.aim += arg,
            Instruction::Up(arg) => self.aim -= arg,
        }
    }
}

enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Instruction {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();

        let instruction = split[0].to_uppercase();
        let argument = split[1].parse()?;

        Ok(match instruction.as_str() {
            "UP" => Instruction::Up(argument),
            "DOWN" => Instruction::Down(argument),
            "FORWARD" => Instruction::Forward(argument),
            _ => panic!("Invalid instruction"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_01(&input), 150);
    }

    #[test]
    fn test_part_02() {
        let input = read_file("example.txt").unwrap();

        assert_eq!(part_02(&input), 900);
    }
}
