use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    println!("Are you experienced? (yes/no)");
    io::stdin().read_line(&mut experience).expect("Failed to read input");

    println!("Enter your age:");
    io::stdin().read_line(&mut age).expect("Failed to read input");

    let experience = experience.trim();
    let age: u32 = age.trim().parse().expect("Input not an integer");

    if experience == "yes" {
        if age >= 40 {
            println!("Annual incentive: N1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Annual incentive: N1,480,000");
        } else if age < 28 {
            println!("Annual incentive: N1,300,000");
        } else {
            println!("The age does not match the given criteria.");
        }
    } else {
        println!("Annual incentive: N100,000");
    }
}