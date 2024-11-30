/// Demonstrates ownership and borrowing in Rust.

pub fn run_ownership() {
    // Ownership
    let s = String::from("hello"); // s owns the string
    println!("s = {}", s);

    // Move
    let t = s; // Ownership moved to t
    // println!("s = {}", s); // Error: s is no longer valid

    // Borrowing
    let u = &t; // u borrows t
    println!("t = {}, u (borrowed) = {}", t, u);

    // Mutable borrow
    let mut v = String::from("mutable");
    {
        let w = &mut v; // w borrows v mutably
        w.push_str(" string");
    }
    println!("v after mutable borrow = {}", v);
}
