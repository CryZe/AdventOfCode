use std::io::prelude::*;
use std::fs::File;

const BASEMENT: isize = -1;
const MOVE_UP: char = '(';
const MOVE_DOWN: char = ')';

struct Santa {
    floor: isize,
    steps: usize,
}

impl Santa {
    fn new() -> Santa {
        Santa {
            floor: 0,
            steps: 0,
        }
    }

    fn switch_floor(&mut self, action: char) {
        match action {
            MOVE_UP => self.floor += 1,
            MOVE_DOWN => self.floor -= 1,
            _ => {}
        };
        self.steps += 1;
    }

    fn walk_until(&mut self, input: &str, target_floor: isize) -> Option<usize> {
        for action in input.chars() {
            self.switch_floor(action);
            if self.floor == target_floor {
                return Some(self.steps);
            }
        }
        None
    }

    fn find_basement(&mut self, input: &str) -> Option<usize> {
        self.walk_until(input, BASEMENT)
    }
}

pub fn santa_functional(input: &str) -> usize {
    input.chars()
         .scan(0, |floor, c| {
             *floor += match c {
                 '(' => 1,
                 ')' => -1,
                 _ => 0,
             };
             Some(*floor)
         })
         .take_while(|&floor| floor != -1)
         .count() + 1
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").expect("File could not be found.");
    file.read_to_string(&mut input).expect("File could not be read.");

    let mut santa = Santa::new();

    let basement_index = santa.find_basement(&input).expect("The Basement wasn't reached.");

    println!("Basement Index: {}", basement_index);
}

#[test]
fn test() {
    assert_eq!(Santa::new().find_basement(")"), Some(1));
    assert_eq!(Santa::new().find_basement("()())"), Some(5));
    assert_eq!(Santa::new().find_basement("()())(((()"), Some(5));
    assert_eq!(Santa::new().find_basement("()()(((()"), None);
}

#[test]
fn test_functional() {
    assert_eq!(santa_functional(")"), 1);
    assert_eq!(santa_functional("()())"), 5);
    assert_eq!(santa_functional("()())(((()"), 5);
    // assert_eq!(santa_functional("()()(((()"), None);
}
