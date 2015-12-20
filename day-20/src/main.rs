pub fn calculate_presents_part1(house: usize) -> usize {
    let sqrt = (house as f32).sqrt() as usize;
    let result = (2..sqrt + 1).fold(house + 1, |acc, i| {
        if house % i == 0 {
            acc + i + house / i
        } else {
            acc
        }
    });
    10 *
    if sqrt * sqrt == house {
        result - sqrt
    } else {
        result
    }
}

pub fn calculate_presents_part2(house: usize) -> usize {
    let sqrt = (house as f32).sqrt() as usize;
    let result = (1..sqrt + 1).fold(0, |acc, i| {
        match (house % i == 0, house / i <= 50, i <= 50) {
            (true, true, true) => acc + i + house / i,
            (true, true, false) => acc + i,
            (true, false, true) => acc + house / i,
            _ => acc,
        }
    });
    11 *
    if sqrt * sqrt == house {
        result - sqrt
    } else {
        result
    }
}

pub fn find_house_part1_slow(minimum_presents: usize) -> usize {
    (1..).find(|h| calculate_presents_part1(*h) >= minimum_presents).unwrap()
}

pub fn find_house_part2_slow(minimum_presents: usize) -> usize {
    (1..).find(|h| calculate_presents_part2(*h) >= minimum_presents).unwrap()
}

fn find_house_part1(minimum_presents: usize) -> usize {
    let div = minimum_presents / 10;
    let mut houses = vec![1; div];

    for elve in 2..div {
        let mut house_id = elve;
        while house_id < div {
            houses[house_id] += elve;
            house_id += elve;
        }
    }

    houses.into_iter().position(|p| p >= div).unwrap()
}

fn find_house_part2(minimum_presents: usize) -> usize {
    let div = minimum_presents / 11;
    let mut houses = vec![0; div];

    for elve in 1..div {
        let mut house_id = elve;
        let mut i = 0;
        while house_id < div && i < 50 {
            houses[house_id] += elve;
            house_id += elve;
            i += 1;
        }
    }

    houses.into_iter().position(|p| p >= div).unwrap()
}

fn main() {
    let presents = 29000000;

    let house = find_house_part1(presents);
    println!("Part 1: House Number {} is the first house that got at least {} presents.",
             house,
             presents);

    let house = find_house_part2(presents);
    println!("Part 2: House Number {} is the first house that got at least {} presents.",
             house,
             presents);
}

#[test]
fn test_calculate_presents_part1() {
    assert_eq!(calculate_presents_part1(1), 10);
    assert_eq!(calculate_presents_part1(2), 30);
    assert_eq!(calculate_presents_part1(3), 40);
    assert_eq!(calculate_presents_part1(4), 70);
    assert_eq!(calculate_presents_part1(5), 60);
    assert_eq!(calculate_presents_part1(6), 120);
    assert_eq!(calculate_presents_part1(7), 80);
    assert_eq!(calculate_presents_part1(8), 150);
    assert_eq!(calculate_presents_part1(9), 130);
}

#[test]
fn test_find_house_part1_slow() {
    assert_eq!(find_house_part1_slow(70), 4);
    assert_eq!(find_house_part1_slow(100), 6);
    assert_eq!(find_house_part1_slow(130), 8);
}

#[test]
fn test_find_house_part1() {
    assert_eq!(find_house_part1(70), 4);
    assert_eq!(find_house_part1(100), 6);
    assert_eq!(find_house_part1(130), 8);
}
