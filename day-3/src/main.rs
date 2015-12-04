use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

use Direction::*;

#[derive(Copy,Clone)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(North),
            '>' => Some(East),
            'v' => Some(South),
            '<' => Some(West),
            _ => None
        }
    }
}

struct Town {
    house_presents: HashMap<(isize, isize), usize>
}

impl Town {
    fn new() -> Town {
        Town {
            house_presents: HashMap::new()
        }
    }

    fn visit_house(&mut self, coordinates: (isize, isize)) -> usize
    {
        let mut presents = self.house_presents.entry(coordinates).or_insert(0);
        *presents += 1;
        *presents
    }

    fn houses_visited(&self) -> usize {
        self.house_presents.len()
    }
}

struct Santa<'town> {
    town: &'town mut Town,
    coordinates: (isize, isize)
}

impl<'town> Santa<'town> {
    fn new(town: &'town mut Town) -> Santa {
        town.visit_house((0, 0));
        Santa {
            town: town,
            coordinates: (0, 0)
        }
    }

    fn visit_town<'a, I>(town: &'town mut Town, directions: I)
        where I: IntoIterator<Item=&'a Direction> {
        let mut santa = Santa::new(town);

        for direction in directions {
            santa.go_to_next_house(*direction);
        }
    }

    fn go_to_next_house(&mut self, dir: Direction) -> usize {
        let (x, y) = self.coordinates;
        self.coordinates = match dir {
            North => (x,     y + 1),
            East =>  (x + 1, y),
            South => (x,     y - 1),
            West =>  (x - 1, y)
        };
        self.town.visit_house(self.coordinates)
    }
}

fn read_file(path: &Path) -> String {
    let mut input = String::new();
    let mut file = File::open(path).expect("File could not be found");
    file.read_to_string(&mut input).expect("File could not be read");
    input
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input.chars().filter_map(|c| Direction::from_char(c)).collect()
}

fn split_directions<I>(directions: I) -> (Vec<Direction>, Vec<Direction>)
    where I: IntoIterator<Item=Direction> {
    let (even, odd): (Vec<_>, Vec<_>) = directions.into_iter().enumerate().partition(|&(i, _)| i % 2 == 0);
    (even.into_iter().map(|(_, d)| d).collect(),
      odd.into_iter().map(|(_, d)| d).collect())
}

fn main() {
    let input = read_file(&Path::new("input.txt"));
    let directions = parse_directions(&input);

    let mut town1 = Town::new();
    Santa::visit_town(&mut town1, &directions);
    println!("Santa visited {} houses", town1.houses_visited());

    let mut town2 = Town::new();
    let (santa, robo_santa) = split_directions(directions);
    Santa::visit_town(&mut town2, &santa);
    Santa::visit_town(&mut town2, &robo_santa);
    println!("Santa and Robo Santa visited {} houses", town2.houses_visited());
}

#[test]
fn test_one_santa()
{
    let mut town1 = Town::new();
    Santa::visit_town(&mut town1, &parse_directions(">"));
    assert_eq!(town1.houses_visited(), 2);

    let mut town2 = Town::new();
    Santa::visit_town(&mut town2, &parse_directions("^>v<"));
    assert_eq!(town2.houses_visited(), 4);

    let mut town3 = Town::new();
    Santa::visit_town(&mut town3, &parse_directions("^v^v^v^v^v"));
    assert_eq!(town3.houses_visited(), 2);
}

#[test]
fn test_two_santas()
{
    let mut town1 = Town::new();
    let (santa1, robo1) = split_directions(parse_directions("^v"));
    Santa::visit_town(&mut town1, &santa1);
    Santa::visit_town(&mut town1, &robo1);
    assert_eq!(town1.houses_visited(), 3);

    let mut town2 = Town::new();
    let (santa2, robo2) = split_directions(parse_directions("^>v<"));
    Santa::visit_town(&mut town2, &santa2);
    Santa::visit_town(&mut town2, &robo2);
    assert_eq!(town2.houses_visited(), 3);

    let mut town3 = Town::new();
    let (santa3, robo3) = split_directions(parse_directions("^v^v^v^v^v"));
    Santa::visit_town(&mut town3, &santa3);
    Santa::visit_town(&mut town3, &robo3);
    assert_eq!(town3.houses_visited(), 11);
}
