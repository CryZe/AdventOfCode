const START_VALUE: u64 = 20151125;

fn index_from_coordinate(row: usize, column: usize) -> usize {
    let diagonal = row + column - 1;
    diagonal * (diagonal - 1) / 2 + column
}

fn get_code(row: usize, column: usize, start: u64) -> u64 {
    let index = index_from_coordinate(row, column);
    let mut value = start;

    for _ in 1..index {
        value = (value * 252533) % 33554393;
    }

    value
}

fn main() {
    let code = get_code(2981, 3075, START_VALUE);
    println!("The Code is {}.", code);
}

#[test]
fn test_index_from_coordinate() {
    assert_eq!(index_from_coordinate(1, 1), 1);
    assert_eq!(index_from_coordinate(2, 1), 2);
    assert_eq!(index_from_coordinate(1, 2), 3);
    assert_eq!(index_from_coordinate(3, 1), 4);
    assert_eq!(index_from_coordinate(2, 2), 5);
    assert_eq!(index_from_coordinate(1, 3), 6);
    assert_eq!(index_from_coordinate(4, 1), 7);
    assert_eq!(index_from_coordinate(3, 2), 8);
    assert_eq!(index_from_coordinate(2, 3), 9);
    assert_eq!(index_from_coordinate(1, 4), 10);
    assert_eq!(index_from_coordinate(5, 1), 11);
}

#[test]
fn test_get_code() {
    assert_eq!(get_code(1, 1, 20151125), 20151125);
    assert_eq!(get_code(1, 2, 20151125), 18749137);
    assert_eq!(get_code(1, 3, 20151125), 17289845);
    assert_eq!(get_code(2, 1, 20151125), 31916031);
    assert_eq!(get_code(2, 2, 20151125), 21629792);
    assert_eq!(get_code(2, 3, 20151125), 16929656);
    assert_eq!(get_code(3, 1, 20151125), 16080970);
    assert_eq!(get_code(3, 2, 20151125), 8057251);
    assert_eq!(get_code(3, 3, 20151125), 1601130);
}
