fn main() {
    for i in 1..=12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            get_day_position(i)
        );
        println!("{}", get_verse(i));

        print!("\n\n");
    }
}

fn get_day_position(dn: i32) -> String {
    let positions = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    return positions.get(dn as usize - 1).unwrap().to_string();
}

fn get_verse(dn: i32) -> String {
    let verses = [
        ["A partridge in a pear tree", "And partridge in a pear tree"][(dn > 1) as usize],
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut verse = String::new();
    for i in (0..dn).rev() {
        verse.push_str(verses[i as usize]);
        verse.push_str("\n");
    }

    return verse;
}
