fn count_bits(n: i64) -> u32 {
    let res = decimal_to_binary(n);
    res.into_iter().filter(|x| *x == 1).count() as u32
}

fn decimal_to_binary(mut n: i64) -> Vec<i64> {
    let mut bin: Vec<i64> = vec![];
    while n > 0 {
        bin.push(n % 2);
        n = n / 2;
    }
    bin
}

fn main() {
    println!("Hello, world!");
}
