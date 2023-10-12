#![deny(clippy::all)]

fn main() {
    // To declare a mutable variable named 'x' with the initial value 5 we need to use the key word mut.
    let mut x = 5;

    // Printing the initial value of 'x'.
    println!("Initial value of x: {}", x);

    // Modifying the value of the mutable variable 'x'.
    x = 10;

    // Printing the modified value of 'x'.
    println!("Modified value of x: {}", x);

    // We can also specify what type of variable it is.
    let mut _strings_only: String = String::from("Only Holds Strings");
}
