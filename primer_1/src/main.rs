/*

Based on: https://github.com/EcoForecast/EF_Activities/blob/master/Exercise_01_RPrimer.Rmd

Scaffolding for this file was generated using ChatGPT and the prompt:
"Generate a small runnable Rust example for a program that assigns the logarithm of
10 to a variable and prints that variable to stdout.
Please include all relevant imports and add comments for what individual lines"
Other questions about e.g. specific library imports were also answered by ChatGPT.
*/

// Import the standard library's f64 type and the `ln` function from the f64 module
// Rust compiler claims this is unnecessary:
// use std::f64;

// ChatGPT
// Import the constant `E` from the `std::f64::consts` module
use std::f64::consts::E;

fn main() {
    /* Question 1:
     1. [A] **Evaluate the following:**
    - a.	ln(1)
    - b.	ln(0)
    - c.	ln(e)
    - d.	ln(-5)
    - e.	-ln(5)
    - f.	ln(1/5)
    - g.	How does R represent when the output of a function is not a number?
     */
    let log_of_1 = 1f64.ln(); // `10f64` specifies that the number 10 is a 64-bit floating-point number
    println!("The natural logarithm of 1 is: {}", log_of_1);
    let log_of_0 = 0f64.ln();
    println!("The natural logarithm of 0 is: {}", log_of_0);
    // e provided courtesy of ChatGPT
    let log_of_e = E.ln();
    println!("The natural logarithm of e is: {}", log_of_e);
    let log_of_minus_5 = (-5f64).ln();
    println!("The natural logarithm of -5 is: {}", log_of_minus_5);
    let minus_log_of_5 = -(5f64.ln());
    println!("The negative natural logarithm of 5 is: {}", minus_log_of_5);
    let log_of_one_fifth = (1f64 / 5f64).ln();
    println!("The natural logarithm of 1/5 is: {}", log_of_one_fifth);

    /*
    2.	[B] **What is the difference between log and log10?**  (Hint: use help!)
    */
    let log10_of_10 = 10f64.log10();
    println!("The base 10 logarithm of 10 is: {}", log10_of_10);
    let log_of_10 = 10f64.ln();
    println!("The natural logarithm of 10 is: {}", log_of_10);
    // Answer: Different base?

    /*
    3.	[A] **Pythagorean theorem**
    - a.	Given a right triangle with sides `a` and `b`, write a few lines of code that will calculate the length of the hypotenuse. Make sure to use variables in this calculations, not hard-coded numbers.
    - b.	Try out your code with `a=5` and `b=13`.
    */
    fn pythagoras(a: f64, b: f64) -> f64 {
        let c: f64 = a.powi(2) + b.powi(2);
        return c.sqrt();
    }
    let a = 5f64;
    let b = 13f64;
    let hyp = pythagoras(a, b);
    println!(
        "The length of the hypotenuse of {} and {} is: {}",
        a, b, hyp
    )

    // TODO continue here
}
