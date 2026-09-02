fn main() {
    let p: f64 = 1000.0;
    let r: f64 = 10.0;
    let t: f64 = 2.0;

    let a = p * (1.0 + (r / 100.0)) * t;
    let simple_interest = a - p;

    println!("Amount is {}", a);
    println!("Simple Interest is {}", simple_interest);
}