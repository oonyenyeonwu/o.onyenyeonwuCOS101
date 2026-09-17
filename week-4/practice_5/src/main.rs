use std::io;

fn main() {
    let mut height = String::new();

    println!("Enter your height in cm:");
    io::stdin().read_line(&mut height).expect("Failed to read input");

    let height: f32 = height.trim().parse().expect("Input not a number");

    if height >= 150.0 && height <= 170.0 {
        println!("Average");
    } else if height > 170.0 && height <= 195.0 {
        println!("Tall");
    } else if height < 150.0 {
        println!("Short");
    } else {
        println!("Abnormal");
    }
}