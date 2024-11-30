/// Demonstrates how to use structs and enums for data modeling.

pub fn run_structs_and_enums() {
    // Define and use a struct
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Struct Example: {:?}", alice);

    // Define and use an enum
    #[derive(Debug)]
    enum Vehicle {
        Car(String),       // Car with a model name
        Bike(String),      // Bike with a model name
        Bus { capacity: u32 }, // Bus with capacity
    }

    let my_car = Vehicle::Car(String::from("Tesla Model S"));
    let school_bus = Vehicle::Bus { capacity: 50 };

    println!("Enum Example: {:?}", my_car);
    println!("Enum Example: {:?}", school_bus);

    // Use match to handle enum variants
    match my_car {
        Vehicle::Car(model) => println!("It's a car: {}", model),
        Vehicle::Bike(model) => println!("It's a bike: {}", model),
        Vehicle::Bus { capacity } => println!("It's a bus with capacity: {}", capacity),
    }
}
