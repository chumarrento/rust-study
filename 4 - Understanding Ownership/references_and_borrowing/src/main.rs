fn main() {
    let s = String::from("hello");

    let len = calculate_length(&s);

    println!("The length of '{s}' is {len}.");

    // This will not work
    // has 2 immutable references
    // r3 has a mutable reference
    // mutating r3 will mutate r1 and r2
    // So Rust does not allow this
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
    // println!("{r1}, {r2}, {r3}");

    // This will work
    // has 2 immutable references
    // but r1 and r2 will not be used after println
    // and this references will be dropped
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1}, {r2}");

    let r3 = &mut s;
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
