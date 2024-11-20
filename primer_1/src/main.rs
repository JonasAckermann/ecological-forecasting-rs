/*
Scaffolding for this file was generated using ChatGPT and the prompt:
"Generate a small runnable Rust example for a program that assigns the logarithm of 
10 to a variable and prints that variable to stdout. 
Please include all relevant imports and add comments for what individual lines"
*/

// Import the standard library's f64 type and the `ln` function from the f64 module
// Rust compiler claims this is unnecessary:
// use std::f64;

fn main() {
    // Calculate the natural logarithm (base e) of 10
    let log_of_10 = 10f64.ln(); // `10f64` specifies that the number 10 is a 64-bit floating-point number

    // Print the result to the standard output
    println!("The natural logarithm of 10 is: {}", log_of_10);
}