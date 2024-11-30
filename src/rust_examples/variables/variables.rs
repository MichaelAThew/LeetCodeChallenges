/// Demonstrates variables, mutability, and data types in Rust.

pub fn run_variables() {
    // Immutable variable
    let x = 10; // Cannot be changed
    println!("x (immutable) = {}", x);

    // Mutable variable
    let mut y = 20; // Can be changed
    println!("y (mutable before change) = {}", y);
    y = 30; // Changing y
    println!("y (mutable after change) = {}", y);

    // Constants
    const MAX_POINTS: u32 = 100_000; // Must be explicitly typed
    println!("MAX_POINTS (constant) = {}", MAX_POINTS);

    // Data types
    let a: i32 = 5; // Signed 32-bit integer
    let b: f64 = 3.14; // 64-bit floating-point
    let is_rust_fun: bool = true; // Boolean
    let letter: char = 'R'; // Character
    println!("a = {}, b = {}, is_rust_fun = {}, letter = {}", a, b, is_rust_fun, letter);

    // Compound types: tuples and arrays
    let tuple: (i32, f64, char) = (42, 6.28, 'T');
    println!("Tuple: {:?}", tuple);

    let array: [i32; 3] = [1, 2, 3];
    println!("Array: {:?}", array);
}
