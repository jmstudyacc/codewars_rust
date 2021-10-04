fn get_count(string: &str) -> usize {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    // chars() = returns an iterator over the chars of a string slice
    // filter() = creates an iterator that determines if an element should be yielded or not
    // contains() = returns true if the slice contains an element with given value
    // count() = consumes the iterator and counting the number of iterations and returning it
    string.chars().filter(|x| vowels.contains(&x)).count()
}

fn get_count_match(string: &str) -> usize {
    string
        .matches(|x| match x {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        })
        .count()
}

fn main() {
    println!("Hello, world!");
}
