fn main() {
    // other_function(5);

    let x = 5;
    // a block is an expression
    // to explicitly return a value from a block, the value to return should not have a semicolon
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let z = five();

    println!("The value of z is: {}", z);
}

// this function follow expression above convention
// to return a value from a function, the value to return should not have a semicolon
fn five() -> i32 {
    // return keyword is optional, is used to return early from a function

    // return 5;
    // if a use semicolon, this will be statement instead of expression
    4 + 1
}

// fn other_function(x: i32) {
//     println!("The value of x is: {}", x);
// }
