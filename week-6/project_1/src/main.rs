use std::io;

fn main() {
    println!("*****************************************");
    println!("*                                       *");
    println!("*   WELCOME TO OUR RESTAURANT!          *");
    println!("*   We are so happy to have you here.   *");
    println!("*                                       *");
    println!("*****************************************");
    println!();

    println!("May we know your name?");
    let mut name_input = String::new();
    io::stdin()
        .read_line(&mut name_input)
        .expect("Failed to read line");
    let name = name_input.trim();

    println!();
    println!("Hello, {}! Welcome, and thank you for choosing us today.", name);
    println!("Please have a look at our delicious menu below.");
    println!();

    println!("============== OUR MENU ==============");
    println!("Code   Food                            Price");
    println!("P      Poundo Yam / Edinkaiko Soup     N3200");
    println!("F      Fried Rice & Chicken            N3000");
    println!("A      Amala & Ewedu Soup              N2500");
    println!("E      Eba & Egusi Soup                N2000");
    println!("W      White Rice & Stew               N2500");
    println!("======================================");
    println!("Special treat: spend more than N10000 and enjoy 5% off!");
    println!();

    println!("{}, what would you like to eat? Enter the food code (P, F, A, E or W):", name);
    let mut choice_input = String::new();
    io::stdin()
        .read_line(&mut choice_input)
        .expect("Failed to read line");

    // trim() removes the Enter key, to_uppercase() lets the user type p or P
    let choice = choice_input.trim().to_uppercase();

    // ---------- 5. Read the quantity ----------
    println!("Great choice! How many plates would you like?");
    let mut quantity_input = String::new();
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read line");

    let quantity: u32 = quantity_input
        .trim()
        .parse()
        .expect("Please enter a whole number");

    let mut food_name = "";
    let mut price: u32 = 0;

    if choice == "P" {
        food_name = "Poundo Yam / Edinkaiko Soup";
        price = 3200;
    } else if choice == "F" {
        food_name = "Fried Rice & Chicken";
        price = 3000;
    } else if choice == "A" {
        food_name = "Amala & Ewedu Soup";
        price = 2500;
    } else if choice == "E" {
        food_name = "Eba & Egusi Soup";
        price = 2000;
    } else if choice == "W" {
        food_name = "White Rice & Stew";
        price = 2500;
    } else {
        println!("Sorry {}, that food code is not on our menu. Please run the program again.", name);
        return; // stop the program here
    }

    let total = price * quantity;

    let mut discount: u32 = 0;

    if total > 10000 {
        discount = total * 5 / 100;
        println!();
        println!("Congratulations, {}! Your order is above N10000, so you get 5% off!", name);
    }

    let amount_to_pay = total - discount;

    println!();
    println!("========== YOUR RECEIPT ==========");
    println!("Customer:    {}", name);
    println!("Food:        {}", food_name);
    println!("Price:       N{}", price);
    println!("Quantity:    {}", quantity);
    println!("Total:       N{}", total);
    println!("Discount:    N{}", discount);
    println!("Amount due:  N{}", amount_to_pay);
    println!("==================================");

    println!();
    println!("Thank you, {}! Your meal is coming right up.", name);
    println!("Enjoy your food, and please come again soon!");
}