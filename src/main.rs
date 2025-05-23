use std::io::{self, Write};
use std::fs;
use std::path::Path;
//use std::process::Command;
//use json::stringify;
use std::process::exit;

fn main() {
    println!("Welcome to the CLI MC-Server Management");
    println!("What would you like to do?");
    println!();
    println!("Actions: ");
    println!("abort: Exits the Application");
    println!("add: Adds a Server via a JSON File");
    println!("install: Install a Server from the Internet");
    println!("help: Lists all Actions");
    println!("start: Start a Server");
    println!();

    loop {
        print!("Input a Keyword: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read the Input");

        let input = input.trim();  // Trim whitespace and newline

        match input {
            "abort" => {
                println!("Exiting application.");
                exit(0);
            }
            "add" => {
                add_server();
            }
            "install" => {
                println!("install: not yet implemented");
            }
            "help" => {
                println!();
                println!("Actions: ");
                println!("abort: Exits the Application");
                println!("add: Adds a Server via a JSON File");
                println!("install: Install a Server from the Internet");
                println!("help: Lists all Actions");
                println!("start: Start a Server");
                println!();
            }
            "start" => {
                println!("start: not yet implemented");
            }
            "easteregg" => {
                println!("You expected to find an Easter Egg here, didn't you?");
                println!("Fine, if you really want one, type iwantaneasteregg as an action.");
            }
            "iwantaneasteregg" => {
                open::that("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
            }
            _ => {
                println!("{} is not a valid Action", input);
            }
        }
    }
}

fn add_server() {
    loop {
        println!("Enter file path: ");
        println!("Type abort to exit the Application");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input_path = String::new();

        io::stdin()
            .read_line(&mut input_path)
            .expect("Failed to read path");

        let path = input_path.trim();

        if path == "abort" {
            println!("Exiting application.");
            exit(0);
        }

        let filetype = Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str());

        if filetype == Some("json") {
            match fs::read_to_string(path) {
                Ok(contents_string) => {
                    println!("File is JSON");
                    // You can parse JSON here or do whatever you want with contents_string
                    break;
                }
                Err(e) => {
                    println!("Failed to read file: {}", e);
                    continue;
                }
            }
        } else {
            println!("File is not JSON! Please enter a Path to a JSON file.");
        }
    }
}
    //println!("test");

    /* let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("Failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("Failed to execute process")
    };

    let hello = String::from_utf8_lossy(&output.stdout);
    println!("Command Output: {}", hello.trim());

    */


