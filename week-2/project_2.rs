fn main() {
    let sales = [
        450_000.0,
        1_500_000.0,
        750_000.0,
        2_850_000.0,
        250_000.0,
    ];

    // Calculate the sum
    let sum: f64 = sales.iter().sum();

    // Calculate the average
    let average = sum / sales.len() as f64;

    println!("Sum of sales = {:.2}", sum);
    println!("Average sales = {:.2}", average);
}
