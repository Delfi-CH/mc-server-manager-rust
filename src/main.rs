//use std::fmt::Error;
use std::io::{self, Read, Write};
use std::fs::{self, File};
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
    println!("exit: Exits the Application");
    println!("init: Looks for a app.cfg file. If this file isnt found, it creats it");
    println!("install: Install a Server from the Internet");
    println!("help: Lists all Actions");
    println!("newcfg: Generates a new app.cfg");
    println!("readcfg: reads the current app.cfg");
    println!("start: Start a Server");
    println!("startjar: Start a Server from a .jar file");

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
                println!("readcfg: reads the current app.cfg");
                println!("start: Start a Server");
                println!("startjar: Start a Server from a .jar file");
            }
            "start" => {
                println!("start: not yet implemented");
            }
            "startjar" => {
                start_manual();
            }
            "newcfg" =>{
                new_cfg();
            }
            "readcfg" => {
                read_cfg();
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
                Ok(_contents_string) => {
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
            cfg_file.write_all(
            check_os().as_bytes()).expect("Could not write to file");
            cfg_file.write_all("Servers: 0".as_bytes()).expect("Could not write to file");
            println!("Servers: 0");
            println!("Finished!");
        }
    }
}

fn new_cfg(){
    match fs::read("app.cfg") {
        Ok(_) => {
            println!("Found app.cfg");
            println!("Removing app.cfg...");
            fs::remove_file("app.cfg").expect("Could not delete file");
            println!("Creating new app.cfg...");
            let mut cfg_file = File::create("app.cfg").expect("Could not create file");
            cfg_file.write_all(check_os().as_bytes()).expect("Could not write to file");
            cfg_file.write_all("Servers: 0".as_bytes()).expect("Could not write to file");
            println!("Servers: 0");
            println!("Finished!");
        }
        Err(_) => {
            println!("app.cfg wasn't found, creating it...");
            let mut cfg_file = File::create("app.cfg").expect("Could not create file");
            cfg_file.write_all(check_os().as_bytes()).expect("Could not write to file");
            cfg_file.write_all("Servers: 0".as_bytes()).expect("Could not write to file");
            println!("Servers: 0");
            println!("Finished!");
        }
    }
}

fn read_cfg() {
    match File::open("app.cfg") {
        Ok(mut app_cfg) => {
            let mut app_cfg_content = String::new();
            if let Err(e) = app_cfg.read_to_string(&mut app_cfg_content) {
                eprintln!("Error reading file: {}", e);
                return;
            }
            println!();
            println!("Contents of app.cfg:");
            println!("-----------------------");
            println!("{}", app_cfg_content);
            println!("-----------------------");
            println!();
        }
        Err(_) => {
            println!("app.cfg not found!");
            println!();
            println!("Would you like to generate a new app.cfg? (y/n)");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut read_cfg_yn = String::new();

            io::stdin()
            .read_line(&mut read_cfg_yn)
            .expect("Could not read the Input");

            let read_cfg_yn= read_cfg_yn.to_lowercase();
            let read_cfg_yn = read_cfg_yn.trim();
            
            if read_cfg_yn == "y" {
                println!("Creating new app.cfg...");
                let mut cfg_file = File::create("app.cfg").expect("Could not create file");
                cfg_file.write_all(check_os().as_bytes()).expect("Could not write to file");
                cfg_file.write_all("Servers: 0".as_bytes()).expect("Could not write to file");
                println!("Servers: 0");
                println!("Finished!");
            } else {
                println!("Aborting...");
            }
        }
    }
}

fn check_os() -> String {
    let info = os_info::get();
    let os_info = format!("OS: {}\n", info);
    println!("OS information: {}", os_info);
    os_info
}

fn start_manual() {
    io::stdout().flush().unwrap();

    println!("Please enter the path to your server.jar");
    print!("> ");

    let mut path_to_jar = String::new();

    io::stdin()
        .read_line(&mut path_to_jar)
        .expect("Could not read the Input");

    let path_to_jar= path_to_jar.to_lowercase();
    let path_to_jar = path_to_jar.trim();

    match fs::read(path_to_jar) {
        Ok(_) => {
            println!("path is real");
        }
        Err(_) => {
            println!("path is false");
        }

}
}