use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;
use std::env::args;
use std::cmp::min;

fn apply_replacement(set: &mut HashSet<String>, molecule: &str, replacement: &(String, String)) {
    let &(ref from, ref to) = replacement;
    for (index, window) in molecule.as_bytes().windows(from.len()).enumerate() {
        if from.starts_with(std::str::from_utf8(window).unwrap()) {
            let mut replaced = molecule[0..index].to_owned();
            replaced.extend(to.chars());
            replaced.extend(molecule[index + from.len()..].chars());
            set.insert(replaced);
        }
    }
}

fn apply_all_replacements(molecule: &str, replacements: &[(String, String)]) -> HashSet<String> {
    let mut set = HashSet::new();
    for replacement in replacements {
        apply_replacement(&mut set, molecule, replacement);
    }
    set
}

fn count_steps(molecule: &str,
               replacements: &[(String, String)],
               max_steps: Option<usize>)
               -> Option<usize> {
    let mut steps = None;

    if let Some(0) = max_steps {
        return None;
    }

    if molecule == "e" {
        return Some(0);
    }

    for end_index in (0..molecule.len()).rev() {
        let cut_molecule = &molecule[..end_index + 1];
        for &(ref from, ref to) in replacements {
            if cut_molecule.ends_with(to) {
                let mut replaced = cut_molecule[..cut_molecule.len() - to.len()].to_owned();
                replaced.extend(from.chars());
                replaced.extend((&molecule[cut_molecule.len()..]).chars());

                let new_max_steps = match (steps, max_steps) {
                    (Some(a), Some(b)) => Some(min(a, b).saturating_sub(2)),
                    (Some(a), None) => Some(a.saturating_sub(2)),
                    (None, Some(b)) => Some(b.saturating_sub(2)),
                    _ => None,
                };

                let inner = count_steps(&replaced, replacements, new_max_steps);

                steps = match (steps, inner) {
                    (Some(a), Some(b)) => Some(min(a, b + 1)),
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b + 1),
                    _ => None,
                };

                // Return early because it's impossible to naively search for the minimum
                if inner != None {
                    return steps;
                }
            }
        }
    }

    steps
}

fn read_file(path: &Path) -> Vec<String> {
    let mut input = String::new();
    let mut file = File::open(path).expect("File could not be found.");
    file.read_to_string(&mut input).expect("File could not be read.");
    input.lines().into_iter().map(|l| l.to_owned()).collect()
}

fn parse_replacements(lines: &[String]) -> Vec<(String, String)> {
    let mut replacements = Vec::new();
    for line in lines.into_iter().take_while(|l| l.len() > 0) {
        let mut splits = line.split_whitespace();
        let replacement = (splits.nth(0).unwrap().to_owned(),
                           splits.nth(1).unwrap().to_owned());
        replacements.push(replacement);
    }
    replacements.sort_by(|&(ref a1, ref b1), &(ref a2, ref b2)| {
        (b2.len() - a2.len()).cmp(&(b1.len() - a1.len()))
    });
    replacements
}

fn parse_molecule(lines: &[String]) -> String {
    lines.into_iter().skip_while(|l| l.len() > 0).nth(1).unwrap().to_owned()
}

fn main() {
    let mut path = PathBuf::from(args().nth(0).unwrap());
    path.pop();
    path.push("input.txt");
    let lines = read_file(&path);
    let replacements = parse_replacements(&lines);
    let molecule = parse_molecule(&lines);

    let set = apply_all_replacements(&molecule, &replacements);
    println!("{} new molecules are possible.", set.len());

    let count = count_steps(&molecule, &replacements, None);
    if let Some(actual_count) = count {
        println!("The molecule can be built in {} steps.", actual_count);
    } else {
        println!("The molecule can't be built.");
    }
}

#[test]
fn test_apply_replacement() {
    let mut set = HashSet::new();
    let molecule = "HOH";
    let replacement = ("H".to_owned(), "HO".to_owned());

    apply_replacement(&mut set, molecule, &replacement);

    assert_eq!(set.contains("HOOH"), true);
    assert_eq!(set.contains("HOHO"), true);
    assert_eq!(set.len(), 2);
}


#[test]
fn test_apply_all_replacements() {
    let molecule = "HOH";
    let replacements = [("H".to_owned(), "HO".to_owned()),
                        ("H".to_owned(), "OH".to_owned()),
                        ("O".to_owned(), "HH".to_owned())];

    let set = apply_all_replacements(molecule, &replacements);

    assert_eq!(set.contains("HOOH"), true);
    assert_eq!(set.contains("HOHO"), true);
    assert_eq!(set.contains("OHOH"), true);
    assert_eq!(set.contains("HHHH"), true);
    assert_eq!(set.len(), 4);
}

#[test]
fn test_count_steps() {
    let replacements = [("e".to_owned(), "H".to_owned()),
                        ("e".to_owned(), "O".to_owned()),
                        ("H".to_owned(), "HO".to_owned()),
                        ("H".to_owned(), "OH".to_owned()),
                        ("O".to_owned(), "HH".to_owned())];

    assert_eq!(count_steps("HOH", &replacements, None), Some(3));
    assert_eq!(count_steps("HOHOHO", &replacements, None), Some(6));
}
