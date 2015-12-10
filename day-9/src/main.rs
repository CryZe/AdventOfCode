use std::collections::{HashSet, HashMap};
use std::cmp;
use std::usize;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;

struct Graph {
    nodes: HashSet<String>,
    connections: HashMap<(String, String), usize>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashSet::new(),
            connections: HashMap::new(),
        }
    }

    fn add_connection(&mut self, a: &str, b: &str, distance: usize) {
        self.connections.entry((a.to_owned(), b.to_owned())).or_insert(distance);
        self.nodes.insert(a.to_owned());
        self.nodes.insert(b.to_owned());
    }

    fn find_route_inner(&self,
                        city: &str,
                        stack: &mut Vec<String>,
                        cmp: &Fn(usize, usize) -> usize)
                        -> Option<usize> {
        stack.push(city.to_owned());
        let connections = self.connections
                              .iter()
                              .filter_map(|c| {
                                  match c {
                                      (&(ref a, ref b), &distance) if a == city => {
                                          Some((b, distance))
                                      }
                                      (&(ref a, ref b), &distance) if b == city => {
                                          Some((a, distance))
                                      }
                                      _ => None,
                                  }
                              })
                              .filter(|&(b, _)| !stack.contains(b))
                              .collect::<Vec<_>>();
        let optimal_distance = if connections.len() == 0 {
            Some(0)
        } else {
            connections.iter().fold(None, |optimal, &(b, distance)| {
                self.find_route_inner(&b, stack, cmp).map(|d| d + distance).map(|d| {
                    match optimal {
                        Some(optimal_value) => cmp(optimal_value, d),
                        None => d,
                    }
                })
            })
        };
        stack.pop();
        optimal_distance
    }

    fn find_route(&self, cmp: &Fn(usize, usize) -> usize) -> Option<usize> {
        let mut stack = Vec::new();
        self.nodes
            .iter()
            .fold(None, |optimal, a| {
                self.find_route_inner(a, &mut stack, cmp).map(|d| {
                    match optimal {
                        Some(optimal_value) => cmp(optimal_value, d),
                        None => d,
                    }
                })
            })
    }

    fn find_shortest_route(&self) -> Option<usize> {
        self.find_route(&cmp::min)
    }

    fn find_longest_route(&self) -> Option<usize> {
        self.find_route(&cmp::max)
    }

    fn parse_connections<'a, I>(&mut self, lines: I)
        where I: IntoIterator<Item = &'a String>
    {
        for line in lines {
            let splits = line.split_whitespace().collect::<Vec<_>>();
            let a = splits[0];
            let b = splits[2];
            let distance = usize::from_str(splits[4]).expect("Couldn't parse the distance");
            self.add_connection(a, b, distance);
        }
    }
}

fn read_file(path: &Path) -> Vec<String> {
    let mut input = String::new();
    let mut file = File::open(path).expect("File could not be found");
    file.read_to_string(&mut input).expect("File could not be read");
    input.lines().into_iter().map(|s| s.to_owned()).collect()
}

fn main() {
    let input = read_file(Path::new("input.txt"));
    let mut graph = Graph::new();
    graph.parse_connections(&input);

    let shortest_route = graph.find_shortest_route();
    println!("Shortest Route is {}",
             shortest_route.map(|x| x.to_string()).unwrap_or("non-existent".to_owned()));

    let longest_route = graph.find_longest_route();
    println!("Longest Route is {}",
             longest_route.map(|x| x.to_string()).unwrap_or("non-existent".to_owned()));
}

#[test]
fn test_shortest() {
    let mut graph = Graph::new();
    graph.add_connection("London", "Dublin", 464);
    graph.add_connection("London", "Belfast", 518);
    graph.add_connection("Dublin", "Belfast", 141);
    let shortest_route = graph.find_shortest_route();
    assert_eq!(shortest_route, Some(605));
}

#[test]
fn test_longest() {
    let mut graph = Graph::new();
    graph.add_connection("London", "Dublin", 464);
    graph.add_connection("London", "Belfast", 518);
    graph.add_connection("Dublin", "Belfast", 141);
    let longest_route = graph.find_longest_route();
    assert_eq!(longest_route, Some(982));
}
