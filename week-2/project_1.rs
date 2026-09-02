fn main() {
    let p: f64 = 520_000_000.0;
    let r: f64 = 10.0;
    let n: i32 = 5;

    // Compound amount: A = P[1 + (R/100)]^n
    let a = p * (1.0 + r / 100.0).powi(n);

    // Compound interest: CI = A - P
    let ci = a - p;

    println!("Amount = ₦{:.2}", a);
    println!("Compound Interest = ₦{:.2}", ci);
}