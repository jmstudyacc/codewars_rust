use std::collections::HashMap;

fn open_close_braces(s: &str) -> bool {
    let mut map = HashMap::new();

    for input in s.chars() {
        match input {
            '(' | '[' | '{' => map.entry(input).or_insert(1),
            _ => continue,
        };
    }

    for input in s.chars() {
        match input {
            ')' => map.entry('(').and_modify(|x| *x -= 1),
            ']' => map.entry('[').and_modify(|x| *x -= 1),
            '}' => map.entry('{').and_modify(|x| *x -= 1),
            _ => continue,
        };
    }
    map.values().all(|z| *z == 0)
}

fn valid_braces(s: &str) -> bool {
    true
}

fn main() {
    println!("Hello, world!");
}
