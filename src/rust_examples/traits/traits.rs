/// Demonstrates traits and generics in Rust.

pub fn run_traits() {
    // Define a trait
    trait Greet {
        fn greet(&self) -> String;
    }

    // Implement the trait for a struct
    struct Dog {
        name: String,
    }

    struct Cat {
        name: String,
    }

    impl Greet for Dog {
        fn greet(&self) -> String {
            format!("Woof! My name is {}", self.name)
        }
    }

    impl Greet for Cat {
        fn greet(&self) -> String {
            format!("Meow! My name is {}", self.name)
        }
    }

    let dog = Dog {
        name: String::from("Buddy"),
    };
    let cat = Cat {
        name: String::from("Whiskers"),
    };

    println!("{}", dog.greet());
    println!("{}", cat.greet());

    // Use a generic function with a trait bound
    fn print_greeting<T: Greet>(animal: T) {
        println!("{}", animal.greet());
    }

    print_greeting(dog);
    print_greeting(cat);
}
