fn main() {
    // Heap and Stack
    // Stack is fast, but limited in size
    // Stack is LIFO, and will be used when the size of the variable is known at compile time
    // Heap is slower, but can grow in size
    // Heap is used when the size of the variable is unknown at compile time
    // Heap will create a pointer to the data on the heap
    // This pointer is stored on the stack
    // With pointer, capacity, and length (both in bytes) of the data on the heap

    // This is a literal string, it is immutable
    // It is stored on the stack
    // So it is fast to create nd can be copied easily
    let s1 = "hello";
    let s2 = s1;

    println!("s1 = {}, s2 = {}", s1, s2);

    // Rust doesnt allow you to copy the value of a variable that is on the heap
    // So shallow or deep copy is not possible
    // When you assign a variable to another variable, the first variable is no longer valid
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}", s1);

    // If you want to copy the value of a variable that is on the heap, you can use the clone method
    // This arbitrary code maybe can be expensive
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // s is valid here
    let s = String::from("hello");

    // get_owner takes ownership of s and s is no longer valid
    get_owner(s);

    let s1 = gives_value(); // gives_value give ownership of the value to s1

    let s2 = String::from("text"); // s2 enters the scope

    let s3 = get_and_give_back(s2); // s2 is moved to get_and_give_back and is no longer valid
}

fn get_owner(s: String) {
    println!("{}", s);
} // s is no longer valid here

fn gives_value() -> String {
    let s = String::from("hello");
    s
} // s is no longer valid here

fn get_and_give_back(s: String) -> String {
    s
} // s is no longer valid here
