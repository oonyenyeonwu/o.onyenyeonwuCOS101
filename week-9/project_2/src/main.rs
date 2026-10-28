use std::io::Write;

fn main() {
    let name = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level = vec!["300", "100", "200", "200", "100"]; 
println!("Student Name, Matric. Number, Department, Level");
for i in 0..name.len() {
        println!("{}, {}, {}, {}", name[i], matric[i], department[i], level[i]);

    let mut file = std::fs::File::create("students.txt").expect("create failed");

    let heading = "Student Name, Matric. Number, Department, Level\n";
    file.write_all(heading.as_bytes()).expect("write failed");


    for i in 0..name.len() {
        file.write_all(name[i].as_bytes()).expect("write failed");
        file.write_all(", ".as_bytes()).expect("write failed");
        file.write_all(matric[i].as_bytes()).expect("write failed");
        file.write_all(", ".as_bytes()).expect("write failed");
        file.write_all(department[i].as_bytes()).expect("write failed");
        file.write_all(", ".as_bytes()).expect("write failed");
        file.write_all(level[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

        file.write_all("\n".as_bytes()).expect("write failed");
    }

    println!("\nData written to file.");
}

