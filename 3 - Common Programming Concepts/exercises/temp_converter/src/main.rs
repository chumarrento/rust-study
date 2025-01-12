fn main() {
    println!("Temperature converter!");

    println!("Choose the conversion type:");

    let choice = get_choice();
    let temperature = get_temperature_to_convert();

    match choice {
        1 => celsius_to_fahrenheit(temperature),
        2 => fahrenheit_to_celsius(temperature),
        _ => println!("Option invalid, restart and try again"),
    }
}

fn get_choice() -> u32 {
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let mut choice = String::new();

    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to save your choice");

    return choice.trim().parse().expect("Please type a number!");
}

fn get_temperature_to_convert() -> i32 {
    println!("Please, provide temperature value:");
    let mut temperature = String::new();

    std::io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to save temperature value");

    return temperature.trim().parse().expect("Please type a number!");
}

fn celsius_to_fahrenheit(celsius: i32) {
    let f = (celsius * 9 / 5) + 32;

    println!("{celsius}°C is {f}°F")
}

fn fahrenheit_to_celsius(fahrenheit: i32) {
    let c = (fahrenheit - 32) * 5 / 9;
    println!("{fahrenheit}°F is {c}°C")
}
