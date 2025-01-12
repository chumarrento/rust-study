fn main() {
    println!("Lets get nth fibonacci number");

    println!("Please provide a positive integer:");

    let mut number = String::new();

    std::io::stdin()
        .read_line(&mut number)
        .expect("Failed read line");

    let number: u32 = number
        .trim()
        .parse()
        .expect("Please provide a valid positive number");

    let result = nth_fibonacci(number);

    println!("The {number}th Fibonacci is {result}");
}

fn nth_fibonacci(number: u32) -> u32 {
    if number <= 1 {
        return number;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=number {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}
