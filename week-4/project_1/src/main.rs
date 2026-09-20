use std::io;

fn main() {
    let mut a_input = String::new();
    let mut b_input = String::new();
    let mut c_input = String::new();

    println!("Enter the value of a:");
    io::stdin().read_line(&mut a_input).expect("Failed to read input");

    println!("Enter the value of b:");
    io::stdin().read_line(&mut b_input).expect("Failed to read input");

    println!("Enter the value of c:");
    io::stdin().read_line(&mut c_input).expect("Failed to read input");

    let a: f32 = a_input.trim().parse().expect("Input not a number");
    let b: f32 = b_input.trim().parse().expect("Input not a number");
    let c: f32 = c_input.trim().parse().expect("Input not a number");

    let discriminant = (b * b) - (4.0 * a * c);

    if discriminant > 0.0 {
        let first_root = (-b + discriminant.sqrt()) / (2.0 * a);
        let second_root = (-b - discriminant.sqrt()) / (2.0 * a);

        println!("The two roots are:");
        println!("First root: {}", first_root);
        println!("Second root: {}", second_root);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);

        println!("There is one real root: {}", root);
    } else {
        println!("There are no real roots.");
    }
}