use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("breweries.txt").expect("create failed");

    file.write_all("Lager\n".as_bytes()).expect("write failed");
    file.write_all("33 Export\n".as_bytes()).expect("write failed");
    file.write_all("Desperados\n".as_bytes()).expect("write failed");
    file.write_all("Goldberg\n".as_bytes()).expect("write failed");
    file.write_all("Gulder\n".as_bytes()).expect("write failed");
    file.write_all("Heineken\n".as_bytes()).expect("write failed");
    file.write_all("Star\n\n".as_bytes()).expect("write failed");

    file.write_all("Stout\n".as_bytes()).expect("write failed");
    file.write_all("Legend\n".as_bytes()).expect("write failed");
    file.write_all("Turbo King\n".as_bytes()).expect("write failed");
    file.write_all("Williams\n\n".as_bytes()).expect("write failed");

    file.write_all("Non-Alcoholic\n".as_bytes()).expect("write failed");
    file.write_all("Maltina\n".as_bytes()).expect("write failed");
    file.write_all("Amstel Malta\n".as_bytes()).expect("write failed");
    file.write_all("Malta Gold\n".as_bytes()).expect("write failed");
    file.write_all("Fayrouz\n".as_bytes()).expect("write failed");

    println!("\nData written to file.");
}