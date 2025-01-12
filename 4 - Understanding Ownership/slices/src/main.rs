fn main() {
    // let mut s = String::from("hello world");

    // let word = first_word(&s);
    // This clear the whole string
    // And the "word" with index of "hello" will not work
    // because the string will be cleared
    // compile will not alert about this
    // s.clear();

    // to fix we can use slice
    let s = String::from("hello world");

    let word = first_word(&s[..]);
    // compile alert about this
    // s.clear() try mutate string creating mutable reference
    // but word have immutable reference
    // borrowing rules!!
    // s.clear();

    println!("The first word is: {}", word);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
