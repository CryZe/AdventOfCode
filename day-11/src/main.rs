fn get_next_string(input: String) -> String {
    let mut bytes = input.into_bytes();
    for i in (0..bytes.len()).rev() {
        match bytes[i] {
            b'z' => bytes[i] = b'a',
            b'a'...b'y' => {
                bytes[i] += 1;
                break;
            }
            _ => panic!("Illegal character found"),
        }
    }
    String::from_utf8(bytes).unwrap()
}

fn contains_increasing_straight(password: &str) -> bool {
    password.as_bytes().windows(3).any(|w| w[2] == w[1] + 1 && w[1] == w[0] + 1)
}

fn doesnt_contain_confusing_letters(password: &str) -> bool {
    !password.chars().any(|c| {
        match c {
            'i' | 'o' | 'l' => true,
            _ => false,
        }
    })
}

fn contains_double_pair(password: &str) -> bool {
    password.as_bytes()
            .windows(2)
            .fold((0, false), |(c, b), w| {
                if !b && w[0] == w[1] {
                    (c + 1, true)
                } else {
                    (c, false)
                }
            })
            .0 >= 2
}

fn is_valid_password(password: &str) -> bool {
    contains_increasing_straight(password) && doesnt_contain_confusing_letters(password) &&
    contains_double_pair(password)
}

fn get_next_password(password: String) -> String {
    let mut password = password;
    loop {
        password = get_next_string(password);
        if is_valid_password(&password) {
            return password;
        }
    }
}

fn main() {
    let password = "cqjxjnds";

    let password = get_next_password(password.to_owned());
    println!("The next password is {}", password);

    let password = get_next_password(password.to_owned());
    println!("The next password is {}", password);
}

#[test]
fn test_get_next_string() {
    assert_eq!(get_next_string("abc".to_owned()), "abd");
    assert_eq!(get_next_string("abcz".to_owned()), "abda");
    assert_eq!(get_next_string("zz".to_owned()), "aa");
}

#[test]
fn test_contains_increasing_straight() {
    assert_eq!(contains_increasing_straight("abc"), true);
    assert_eq!(contains_increasing_straight("bcd"), true);
    assert_eq!(contains_increasing_straight("cde"), true);
    assert_eq!(contains_increasing_straight("abd"), false);
    assert_eq!(contains_increasing_straight("hijklmmn"), true);
    assert_eq!(contains_increasing_straight("abbceffg"), false);
}

#[test]
fn test_doesnt_contain_confusing_letters() {
    assert_eq!(doesnt_contain_confusing_letters("acdfeffv"), true);
    assert_eq!(doesnt_contain_confusing_letters("sdfdsfok"), false);
    assert_eq!(doesnt_contain_confusing_letters("dsfdsfdsisdf"), false);
    assert_eq!(doesnt_contain_confusing_letters("sdfdsfeflasd"), false);
    assert_eq!(doesnt_contain_confusing_letters("hijklmmn"), false);
}

#[test]
fn test_contains_double_pair() {
    assert_eq!(contains_double_pair("abbceffg"), true);
    assert_eq!(contains_double_pair("abbcegjk"), false);
    assert_eq!(contains_double_pair("abcdeggg"), false);
}

#[test]
fn test_is_valid_password() {
    assert_eq!(is_valid_password("abcdffaa"), true);
    assert_eq!(is_valid_password("ghjaabcc"), true);
    assert_eq!(is_valid_password("abbcegjk"), false);
    assert_eq!(is_valid_password("hijklmmn"), false);
    assert_eq!(is_valid_password("acdfeffv"), false);
}

#[test]
fn test_get_next_password() {
    assert_eq!(get_next_password("abcdefgh".to_owned()), "abcdffaa");
    assert_eq!(get_next_password("ghijklmn".to_owned()), "ghjaabcc");
}
