use std::io;

fn read_number(message: &str) -> f64 {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let number: f64 = input.trim().parse().expect("Please enter a number");
    return number;
}

fn trapezium_area() -> f64 {
    let height = read_number("Enter the height:");
    let base1 = read_number("Enter the first base:");
    let base2 = read_number("Enter the second base:");
    return height / 2.0 * (base1 + base2);
}

fn rhombus_area() -> f64 {
    let diagonal1 = read_number("Enter the first diagonal:");
    let diagonal2 = read_number("Enter the second diagonal:");
    return 0.5 * diagonal1 * diagonal2;
}

fn parallelogram_area() -> f64 {
    let base = read_number("Enter the base:");
    let altitude = read_number("Enter the altitude:");
    return base * altitude;
}

fn cube_surface_area() -> f64 {
    let side = read_number("Enter the side length:");
    return 6.0 * side * side;
}

fn cylinder_volume() -> f64 {
    let pi: f64 = 3.14159;
    let radius = read_number("Enter the radius:");
    let height = read_number("Enter the height:");
    return pi * radius * radius * height;
}

fn main() {
    println!("*************************************************");
    println!("*                                               *");
    println!("*        WELCOME TO THE SHAPE CALCULATOR!       *");
    println!("*                                               *");
    println!("*************************************************");
    println!();
    println!("A warm welcome to our MTH 101 lecturer, we are honoured to have you!");
    println!("Welcome also to all students, teachers, builders, designers");
    println!("and anyone who needs a quick answer to a shape problem.");
    println!("Let us do the maths for you.");
    println!();
    println!("Please choose a shape:");
    println!("1. Trapezium     (area)");
    println!("2. Rhombus       (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube          (surface area)");
    println!("5. Cylinder      (volume)");
    println!();

    let choice = read_number("Enter your choice (1 to 5):");
    println!();

    if choice == 1.0 {
        let answer = trapezium_area();
        println!("The area of the trapezium is {}", answer);
    } else if choice == 2.0 {
        let answer = rhombus_area();
        println!("The area of the rhombus is {}", answer);
    } else if choice == 3.0 {
        let answer = parallelogram_area();
        println!("The area of the parallelogram is {}", answer);
    } else if choice == 4.0 {
        let answer = cube_surface_area();
        println!("The surface area of the cube is {}", answer);
    } else if choice == 5.0 {
        let answer = cylinder_volume();
        println!("The volume of the cylinder is {}", answer);
    } else {
        println!("Sorry, that is not a valid choice. Please run the program again.");
        return;
    }

    println!();
    println!("Thank you for using the Shape Calculator. Goodbye!");
}