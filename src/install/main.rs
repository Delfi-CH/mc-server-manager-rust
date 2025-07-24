// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::{self, Write};
fn main() {
    println!("Hello World! Installer bin running.");
    println!("APPNAME Installer V0.1");
    println!("Option 1:");
    println!("What Packages do you want to install?");
    println!("1 | Webapp-Backend [DEFAULT]");
    println!("2 | Command-Line App");
    println!("3 | All");

    let input1 = 0;
    
    loop {

    print!("->| ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read the Input");

        let input= input.to_lowercase();
        let input = input.trim();

    if input == "1" {
        println!("Selecting Webapp-Backend...");
        let input1 = 1;
        break;
    } else if input == "2" {
        println!("Selecting Command-Line App...");
        let input1 = 2;
        break;
    } else if input == "3" {
        println!("Selecting both options...");
        let input1 = 3;
        break;
    } else if input == "" {
        println!("Selecting Webapp-Backend...");
        let input1 = 1;
        break;
    } else {
        println!("Not a valid input. Please try again.");
    }  
}
}