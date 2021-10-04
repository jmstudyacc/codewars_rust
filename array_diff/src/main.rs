fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    // creating an empty vec![] to store the result
    let mut c = a;

    // retain() = retains only the elements specified by the predicate
    // contains() = returns true if the slice contains an element with the given value
    c.retain(|x| !b.contains(&x));
    c
}

fn array_diff_iter<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    // into_iter() = creates a consuming iterator, the vector cannot be called after using this
    // filter() = creates an iterator that determines if an element should be yielded or not
    // contains() = returns true if the slice contains an element with the given value
    // collect() = transforms an iterator into a collection
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

fn main() {
    println!("Hello, world!");
}
