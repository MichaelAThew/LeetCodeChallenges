/// Demonstrates error handling in Rust with `Result` and `Option`.

pub fn run_error_handling() {
    // Option Example
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    println!(
        "Option Example: {:?}, {:?}",
        some_number.unwrap_or(0),
        no_number.unwrap_or(-1)
    );

    // Result Example
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(a / b)
        }
    }

    match divide(10, 2) {
        Ok(result) => println!("Result of division: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    match divide(10, 0) {
        Ok(result) => println!("Result of division: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    // Using `?` operator for cleaner error propagation
    fn try_divide(a: i32, b: i32) -> Result<i32, String> {
        let result = divide(a, b)?;
        Ok(result * 2) // Do something with the result
    }

    match try_divide(20, 2) {
        Ok(result) => println!("Doubled result: {}", result),
        Err(err) => println!("Error with ? operator: {}", err),
    }
}
