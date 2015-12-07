use std::collections::HashMap;
use std::ops::Not;
use std::str::FromStr;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use Expression::*;

struct CPU {
    registers: HashMap<String, Expression>,
    cache: HashMap<String, Option<u16>>,
}

#[derive(Debug, PartialEq, Clone)]
enum Expression {
    Register(String),
    Literal(u16),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    LShift(Box<Expression>, Box<Expression>),
    RShift(Box<Expression>, Box<Expression>),
    Complement(Box<Expression>),
}

#[derive(Debug, PartialEq)]
struct Instruction {
    target_register: String,
    expression: Expression,
}

impl CPU {
    fn new() -> Self {
        CPU {
            registers: HashMap::new(),
            cache: HashMap::new(),
        }
    }

    fn insert(&mut self, instruction: Instruction) {
        self.registers.insert(instruction.target_register, instruction.expression);
    }

    fn clear_cache(&mut self) {
        self.cache.clear();
    }

    fn set_cache_value(&mut self, register: &str, value: Option<u16>) {
        self.cache.insert(register.to_owned(), value);
    }

    fn evaluate(&mut self, register: &str) -> Option<u16> {
        if self.cache.contains_key(register) {
            self.cache.get(register).cloned().unwrap_or(None)
        } else {
            self.set_cache_value(register, None);
            let result = self.registers.get(register).cloned().and_then(|r| r.evaluate(self));
            self.set_cache_value(register, result);
            result
        }
    }

    fn insert_all<'a, I>(&mut self, instructions: I)
        where I: IntoIterator<Item = Instruction>
    {
        for instruction in instructions {
            self.insert(instruction);
        }
    }
}

impl Expression {
    fn parse(line: &str) -> Expression {
        if let Some(index) = line.find(" AND ") {
            let split = line.split_at(index);
            let left = Self::parse(split.0);
            let right = Self::parse(&split.1[5..]);
            And(Box::new(left), Box::new(right))
        } else if let Some(index) = line.find(" OR ") {
            let split = line.split_at(index);
            let left = Self::parse(split.0);
            let right = Self::parse(&split.1[4..]);
            Or(Box::new(left), Box::new(right))
        } else if let Some(index) = line.find(" LSHIFT ") {
            let split = line.split_at(index);
            let left = Self::parse(split.0);
            let right = Self::parse(&split.1[8..]);
            LShift(Box::new(left), Box::new(right))
        } else if let Some(index) = line.find(" RSHIFT ") {
            let split = line.split_at(index);
            let left = Self::parse(split.0);
            let right = Self::parse(&split.1[8..]);
            RShift(Box::new(left), Box::new(right))
        } else if line.starts_with("NOT ") {
            let inner = Self::parse(&line[4..]);
            Complement(Box::new(inner))
        } else if let Ok(literal) = u16::from_str(line) {
            Literal(literal)
        } else {
            Register(line.to_owned())
        }
    }

    fn evaluate(&self, cpu: &mut CPU) -> Option<u16> {
        match self {
            &Register(ref register) => cpu.evaluate(&register),
            &Literal(literal) => Some(literal),
            &And(ref left, ref right) => {
                left.evaluate(cpu).and_then(|l| right.evaluate(cpu).map(|r| l & r))
            }
            &Or(ref left, ref right) => {
                left.evaluate(cpu).and_then(|l| right.evaluate(cpu).map(|r| l | r))
            }
            &LShift(ref left, ref right) => {
                left.evaluate(cpu)
                    .and_then(|l| right.evaluate(cpu).map(|r| l.wrapping_shl(r as u32)))
            }
            &RShift(ref left, ref right) => {
                left.evaluate(cpu)
                    .and_then(|l| right.evaluate(cpu).map(|r| l.wrapping_shr(r as u32)))
            }
            &Complement(ref expression) => expression.evaluate(cpu).map(|v| v.not()),
        }
    }
}

impl Instruction {
    fn new(register: &str, expression: Expression) -> Self {
        Instruction {
            target_register: register.to_owned(),
            expression: expression,
        }
    }

    fn parse(line: &str) -> Self {
        let index = line.find(" -> ").unwrap();
        let splits = line.split_at(index);
        let expression = Expression::parse(splits.0);
        let register = &splits.1[4..];
        Instruction::new(register, expression)
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
    let mut cpu = CPU::new();

    cpu.insert_all(instructions);
    let value_of_a = cpu.evaluate("a");

    println!("Value of Register 'a' for Part 1: {}",
             value_of_a.map(|x| x.to_string()).unwrap_or("-".to_owned()));

    cpu.clear_cache();
    cpu.set_cache_value("b", value_of_a);

    let new_value_of_a = cpu.evaluate("a");

    println!("Value of Register 'a' for Part 2: {}",
             new_value_of_a.map(|x| x.to_string()).unwrap_or("-".to_owned()));
}

#[test]
fn test_parse() {
    assert_eq!(Instruction::parse("123 -> x"),
               Instruction::new("x", Literal(123)));
    assert_eq!(Instruction::parse("456 -> y"),
               Instruction::new("y", Literal(456)));
    assert_eq!(Instruction::parse("x AND y -> d"),
               Instruction::new("d",
                                And(Box::new(Register("x".to_owned())),
                                    Box::new(Register("y".to_owned())))));
    assert_eq!(Instruction::parse("x OR y -> e"),
               Instruction::new("e",
                                Or(Box::new(Register("x".to_owned())),
                                   Box::new(Register("y".to_owned())))));
    assert_eq!(Instruction::parse("x LSHIFT 2 -> f"),
               Instruction::new("f",
                                LShift(Box::new(Register("x".to_owned())), Box::new(Literal(2)))));
    assert_eq!(Instruction::parse("y RSHIFT 2 -> g"),
               Instruction::new("g",
                                RShift(Box::new(Register("y".to_owned())), Box::new(Literal(2)))));
    assert_eq!(Instruction::parse("NOT x -> h"),
               Instruction::new("h", Complement(Box::new(Register("x".to_owned())))));
    assert_eq!(Instruction::parse("NOT y -> i"),
               Instruction::new("i", Complement(Box::new(Register("y".to_owned())))));
}

#[test]
fn test_execute() {
    let mut cpu = CPU::new();
    cpu.insert(Instruction::parse("NOT y -> i"));
    cpu.insert(Instruction::parse("x OR y -> e"));
    cpu.insert(Instruction::parse("123 -> x"));
    cpu.insert(Instruction::parse("456 -> y"));
    cpu.insert(Instruction::parse("x AND y -> d"));
    cpu.insert(Instruction::parse("x LSHIFT 2 -> f"));
    cpu.insert(Instruction::parse("y RSHIFT 2 -> g"));
    cpu.insert(Instruction::parse("NOT x -> h"));
    assert_eq!(cpu.evaluate("d"), Some(72));
    assert_eq!(cpu.evaluate("e"), Some(507));
    assert_eq!(cpu.evaluate("f"), Some(492));
    assert_eq!(cpu.evaluate("g"), Some(114));
    assert_eq!(cpu.evaluate("h"), Some(65412));
    assert_eq!(cpu.evaluate("i"), Some(65079));
    assert_eq!(cpu.evaluate("x"), Some(123));
    assert_eq!(cpu.evaluate("y"), Some(456));
}

#[test]
fn test_loop_handling() {
    let mut cpu = CPU::new();
    cpu.insert(Instruction::parse("5 AND x -> a"));
    cpu.insert(Instruction::parse("NOT x -> b"));
    cpu.insert(Instruction::parse("y AND b -> x"));
    cpu.insert(Instruction::parse("456 -> y"));
    assert_eq!(cpu.evaluate("a"), None);
}
