use std::fmt::Error;
use std::io::{self, Write};
use std::fs::{self, File};
use std::path::Path;
//use std::process::Command;
//use json::stringify;
use std::process::exit;
use std::ptr::read;
use os_info::get;

fn main() {

    println!("Welcome to the CLI MC-Server Management");
    println!("What would you like to do?");
    println!();
    println!("Actions: ");
    println!("abort: Exits the Application");
    println!("add: Adds a Server via a JSON File");
    println!("exit: Exits the Application");
    println!("init: Looks for a app.cfg file. If this file isnt found, it creats it");
    println!("install: Install a Server from the Internet");
    println!("help: Lists all Actions");
    println!("newcfg: Generates a new app.cfg");
    println!("start: Start a Server");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read the Input");

        let input= input.to_lowercase();
        let input = input.trim();

        match input {
            "abort" => {
                println!("Exiting application.");
                exit(0);
            }
            "add" => {
                add_server();
            }
            "exit" =>{
                println!("Exiting application.");
                exit(0); 
            }
            "init" =>{
                init();
            }
            "install" => {
                println!("install: not yet implemented");
            }
            "help" => {
                println!();
                println!("Actions: ");
                println!("abort: Exits the Application");
                println!("add: Adds a Server via a JSON File");
                println!("exit: Exits the Application");
                println!("init: Looks for a app.cfg file. If this file isnt found, it creats it");
                println!("install: Install a Server from the Internet");
                println!("help: Lists all Actions");
                println!("newcfg: Generates a new app.cfg");
                println!("start: Start a Server");
            }
            "start" => {
                println!("start: not yet implemented");
            }
            "newcfg" =>{
                new_cfg();
            }
            "easteregg" => {
                println!("You expected to find an Easter Egg here, didn't you?");
                println!("Fine, if you really want one, type iwantaneasteregg as an action.");
            }
            "iwantaneasteregg" => {
                if let Err(e) = open::that("https://www.youtube.com/watch?v=dQw4w9WgXcQ") {
                eprintln!("Failed to open browser: {}", e);
                }
                exit(69)
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
fn init() {
    match fs::read("app.cfg") {
        Ok(_) => {
            println!("Found app.cfg");
        }
        Err(_) => {
            println!("app.cfg wasn't found, creating it...");
            let mut cfg_file = File::create("app.cfg").expect("Could not create file");
            cfg_file.write_all(check_os().as_bytes()).expect("Could not write to file");
        }
    }
}

fn new_cfg(){
    match fs::read("app.cfg") {
        Ok(_) => {
            println!("Found app.cfg");
            println!("Removing app.cfg...");
            fs::remove_file("app.cfg");
            println!("Creating new app.cfg...");
            let mut cfg_file = File::create("app.cfg").expect("Could not create file");
            cfg_file.write_all(check_os().as_bytes()).expect("Could not write to file");
        }
        Err(_) => {
            println!("app.cfg wasn't found, creating it...");
            let mut cfg_file = File::create("app.cfg").expect("Could not create file");
            cfg_file.write_all(check_os().as_bytes()).expect("Could not write to file");
        }
    }
}

fn check_os() -> String {
    let info = os_info::get();
    let os_info = format!("OS: {}\n", info);
    println!("OS information: {}", os_info);
    os_info
}