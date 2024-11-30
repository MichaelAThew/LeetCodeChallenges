/// Demonstrates control flow in Rust: if, match, and loops.

pub fn run_control_flow() {
    // if-else statements
    let number = 5;
    if number > 0 {
        println!("{} is positive", number);
    } else {
        println!("{} is negative", number);
    }

    // match expression
    let grade = 'A';
    match grade {
        'A' => println!("Excellent!"),
        'B' => println!("Good job!"),
        'C' => println!("You passed."),
        _ => println!("Invalid grade."),
    }

    // Loops
    // while loop
    let mut counter = 3;
    while counter > 0 {
        println!("Counter: {}", counter);
        counter -= 1;
    }

    // for loop
    for i in 1..4 {
        println!("For loop iteration: {}", i);
    }

    // loop keyword
    let mut n = 0;
    loop {
        if n == 3 {
            break;
        }
        println!("Infinite loop iteration: {}", n);
        n += 1;
    }
}
