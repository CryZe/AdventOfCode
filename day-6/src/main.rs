use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::cmp;

use Operation::*;

struct BinaryLights {
    lights: Vec<bool>,
}

struct DimmableLights {
    lights: Vec<isize>,
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Toggle,
    TurnOff,
    TurnOn,
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    operation: Operation,
    min: (usize, usize),
    max: (usize, usize),
}

impl Instruction {
    fn parse_coordinate(pair: &str) -> (usize, usize) {
        let index = pair.find(',').unwrap();
        let splits = pair.split_at(index);
        let x = usize::from_str(splits.0).unwrap();
        let y = usize::from_str(&splits.1[1..]).unwrap();
        (x, y)
    }

    fn parse(line: &str) -> Self {
        let mut splits = line.split_whitespace();
        let op1 = splits.next().unwrap();
        let operation = if op1 == "turn" {
            let op2 = splits.next().unwrap();
            if op2 == "on" {
                TurnOn
            } else {
                TurnOff
            }
        } else {
            Toggle
        };
        let min = Self::parse_coordinate(splits.next().unwrap());
        let _through = splits.next();
        let max = Self::parse_coordinate(splits.next().unwrap());
        Instruction {
            operation: operation,
            min: min,
            max: max,
        }
    }
}

impl BinaryLights {
    fn new() -> Self {
        BinaryLights { lights: vec![false; 1_000_000] }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        for y in instruction.min.0..instruction.max.0 + 1 {
            for x in instruction.min.1..instruction.max.1 + 1 {
                self.lights[x + 1000 * y] = match instruction.operation {
                    TurnOn => true,
                    TurnOff => false,
                    Toggle => !self.lights[x + 1000 * y],
                };
            }
        }
    }

    fn apply_instructions<'a, I>(&mut self, instructions: I)
        where I: IntoIterator<Item = &'a Instruction>
    {
        for instruction in instructions {
            self.apply_instruction(instruction);
        }
    }

    fn get_active_lights_count(&self) -> usize {
        self.lights.iter().filter(|&b| *b).count()
    }
}

impl DimmableLights {
    fn new() -> Self {
        DimmableLights { lights: vec![0; 1_000_000] }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        for y in instruction.min.0..instruction.max.0 + 1 {
            for x in instruction.min.1..instruction.max.1 + 1 {
                self.lights[x + 1000 * y] = cmp::max(0,
                                                     self.lights[x + 1000 * y] +
                                                     match instruction.operation {
                                                         TurnOn => 1,
                                                         TurnOff => -1,
                                                         Toggle => 2,
                                                     });
            }
        }
    }

    fn apply_instructions<'a, I>(&mut self, instructions: I)
        where I: IntoIterator<Item = &'a Instruction>
    {
        for instruction in instructions {
            self.apply_instruction(instruction);
        }
    }

    fn get_active_lights_count(&self) -> isize {
        self.lights.iter().fold(0, |a, &l| a + l as isize)
    }
}

fn read_file(path: &Path) -> Vec<String> {
    let mut input = String::new();
    let mut file = File::open(path).expect("File could not be found");
    file.read_to_string(&mut input).expect("File could not be read");
    input.lines().into_iter().map(|s| s.to_owned()).collect()
}

fn parse_instructions<'a, I>(lines: I) -> Vec<Instruction>
    where I: IntoIterator<Item = &'a String>
{
    lines.into_iter().map(|l| Instruction::parse(l)).collect()
}

fn main() {
    let lines = read_file(Path::new("input.txt"));
    let instructions = parse_instructions(&lines);

    let mut binary_lights = BinaryLights::new();
    binary_lights.apply_instructions(&instructions);
    let active_binary_lights = binary_lights.get_active_lights_count();
    println!("Active Binary Lights: {}", active_binary_lights);

    let mut dimmable_lights = DimmableLights::new();
    dimmable_lights.apply_instructions(&instructions);
    let active_dimmable_lights = dimmable_lights.get_active_lights_count();
    println!("Active Dimmable Lights: {}", active_dimmable_lights);
}

#[test]
fn test_parsing() {
    assert_eq!(Instruction::parse("turn on 0,0 through 999,999"),
               Instruction {
                   operation: TurnOn,
                   min: (0, 0),
                   max: (999, 999),
               });
    assert_eq!(Instruction::parse("toggle 0,0 through 999,0"),
               Instruction {
                   operation: Toggle,
                   min: (0, 0),
                   max: (999, 0),
               });
    assert_eq!(Instruction::parse("turn off 499,499 through 500,500"),
               Instruction {
                   operation: TurnOff,
                   min: (499, 499),
                   max: (500, 500),
               });
}

#[test]
fn test_instructions_on_binary_lights() {
    let mut lights = BinaryLights::new();
    let instruction1 = Instruction {
        operation: TurnOn,
        min: (0, 0),
        max: (999, 999),
    };
    let instruction2 = Instruction {
        operation: Toggle,
        min: (0, 0),
        max: (999, 0),
    };
    let instruction3 = Instruction {
        operation: TurnOff,
        min: (499, 499),
        max: (500, 500),
    };

    lights.apply_instruction(&instruction1);
    assert_eq!(lights.get_active_lights_count(), 1000000);

    lights.apply_instruction(&instruction2);
    assert_eq!(lights.get_active_lights_count(), 999000);

    lights.apply_instruction(&instruction3);
    assert_eq!(lights.get_active_lights_count(), 998996);
}


#[test]
fn test_instructions_on_dimmable_lights() {
    let mut lights = DimmableLights::new();
    let instruction1 = Instruction {
        operation: TurnOn,
        min: (0, 0),
        max: (999, 999),
    };
    let instruction2 = Instruction {
        operation: Toggle,
        min: (0, 0),
        max: (999, 0),
    };
    let instruction3 = Instruction {
        operation: TurnOff,
        min: (499, 499),
        max: (500, 500),
    };

    lights.apply_instruction(&instruction1);
    assert_eq!(lights.get_active_lights_count(), 1000000);

    lights.apply_instruction(&instruction2);
    assert_eq!(lights.get_active_lights_count(), 1002000);

    lights.apply_instruction(&instruction3);
    assert_eq!(lights.get_active_lights_count(), 1001996);
}
