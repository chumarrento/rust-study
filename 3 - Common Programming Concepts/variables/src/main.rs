fn main() {
    // Const as Const
    // like other languages...
    // const PI : f32 = 3.14159;

    // mutability
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // print!("The value of x is: {}", x);

    // shadowing
    // Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword.
    // By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    // In shadowing, we can change the type of the value but reuse the same name of the variable different from mut.
    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
}
