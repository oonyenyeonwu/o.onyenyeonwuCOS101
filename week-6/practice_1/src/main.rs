fn main() {
    let name = "Aisha Lawal";
    let uni: &str = "Pan-Atlantic University";
    let department: &'static str = "Computer Science";
    let address: &'static str = "Lekki, Lagos";
    let school: &'static str = "School of Science and Technology";

    println!("{}", name);
    println!("{}", uni);
    println!("{}", department);
    println!("{}", address);
    println!("{}", school);
}