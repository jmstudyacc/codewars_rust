use regex::Regex;

fn validate_pin(pin: &str) -> bool {
    pin.chars().all(|x| x.is_digit(10)) && (pin.len() == 4 || pin.len() == 6)
}

fn validate_pin_regex(pin: &str) -> bool {
    Regex::new(r"^((\d{4})|(\d{6}))$").unwrap().is_match(pin)
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn invalid_length_tests() {
    assert_eq!(validate_pin("1"), false);
    assert_eq!(validate_pin("12"), false);
    assert_eq!(validate_pin("123"), false);
    assert_eq!(validate_pin("12345"), false);
    assert_eq!(validate_pin("1234567"), false);
    assert_eq!(validate_pin("-1234"), false);
    assert_eq!(validate_pin("1.234"), false);
    assert_eq!(validate_pin("-1.234"), false);
    assert_eq!(validate_pin("00000000"), false);
}

#[test]
fn valid_pin_tests() {
    assert_eq!(validate_pin("1234"), true);
    assert_eq!(validate_pin("0000"), true);
    assert_eq!(validate_pin("1111"), true);
    assert_eq!(validate_pin("123456"), true);
    assert_eq!(validate_pin("098765"), true);
    assert_eq!(validate_pin("000000"), true);
    assert_eq!(validate_pin("123456"), true);
    assert_eq!(validate_pin("090909"), true);
}
