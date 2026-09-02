fn main() {
    let p: f64 = 210_000.0;
    let r: f64 = 5.0;
    let n: i32 = 3;

    // Depreciation formula: A = P[1 - (R/100)]^n
    let a = p * (1.0 - r / 100.0).powi(n);

    println!("Value of the TV after 3 years = N{:.2}", a);
}