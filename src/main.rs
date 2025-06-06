use std::f32::consts::E;
//use std::fmt::Error;
use std::io::{self, Read, Write};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
//use json::stringify;
use std::process::exit;

fn main() {

    println!("Welcome to the CLI MC-Server Management");
    println!("What would you like to do?");
    println!();
    println!("Actions: ");
    println!("abort: Exits the Application");
    println!("add: Adds a Server via a JSON File");
    println!("check: Checks if Java is installed on the System");
    println!("exit: Exits the Application");
    println!("init: Looks for a app.cfg file. If this file isnt found, it creats it");
    println!("install: Install a Server from the Internet");
    println!("help: Lists all Actions");
    println!("newcfg: Generates a new app.cfg");
    println!("readcfg: reads the current app.cfg");
    println!("source: Opens Git Repository in your default Browser");
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
            "check" => {
                check_java();
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
                println!("Actions: ");
                println!("abort: Exits the Application");
                println!("add: Adds a Server via a JSON File");
                println!("check: Checks if Java is installed on the System");
                println!("exit: Exits the Application");
                println!("init: Looks for a app.cfg file. If this file isnt found, it creats it");
                println!("install: Install a Server from the Internet");
                println!("help: Lists all Actions");
                println!("newcfg: Generates a new app.cfg");
                println!("readcfg: reads the current app.cfg");
                println!("source: Opens Git Repository in your default Browser");
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
            "source" => {
                if let Err(e) = open::that("https://github.com/Delfi-CH/mc-server-management/tree/main") {
                eprintln!("Failed to open browser: {}", e);
                }
            }
            _ => {
                println!("'{}' is not a valid Action", input);
            }
        }
    }
}

fn add_server() {
    loop {
        println!("Enter file path: ");
        println!("Type abort to exit.");
        print!("-> ");
        io::stdout().flush().unwrap();

        let mut input_path = String::new();

        io::stdin()
            .read_line(&mut input_path)
            .expect("Failed to read path");

        let path = input_path.trim();

        if path == "abort" {
            break;
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

fn new_cfg_silent(){
    // same as fn new_cfg, but doesnt print output
    match fs::read("app.cfg") {
        Ok(_) => {
            fs::remove_file("app.cfg").expect("Could not delete file");
            let mut cfg_file = File::create("app.cfg").expect("Could not create file");
            cfg_file.write_all(check_os().as_bytes()).expect("Could not write to file");
            cfg_file.write_all("Servers: 0".as_bytes()).expect("Could not write to file");
        }
        Err(_) => {
            let mut cfg_file = File::create("app.cfg").expect("Could not create file");
            cfg_file.write_all(check_os().as_bytes()).expect("Could not write to file");
            cfg_file.write_all("Servers: 0".as_bytes()).expect("Could not write to file");
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
            print!("-> ");
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
fn read_cfg_silent() -> String {

    // same as fn read_cfg, but doesnt print output
    match File::open("app.cfg") {
        Ok(mut app_cfg) => {
            let mut app_cfg_content = String::new();
            if let Err(e) = app_cfg.read_to_string(&mut app_cfg_content) {
                eprintln!("Error reading file: {}", e);
                return app_cfg_content;
            } else {
                return app_cfg_content;
            }
        }
        Err(_) => {
            println!("app.cfg not found!");
            println!("Generating new app.cfg...");
            new_cfg_silent();
            let return_error_statement = "rerun";
            return return_error_statement.to_string();        
        }
    }
}

fn check_os() -> String {
    let info = os_info::get();
    let os_info = format!("OS: {}\n", info);
    println!("OS information: {}", os_info);
    os_info
}

fn check_java() -> (String, bool){ 
    let mut os_name = read_cfg_silent();
    let mut has_java = false;
    while os_name == "rerun" {
            os_name = read_cfg_silent();
    }
    if os_name.contains("Windows") {
        let output = Command::new("java")
            .args(&["-version"])
            .output()
            .expect("Failed to check for Java");
        let java_info = String::from_utf8_lossy(&output.stderr);       
       if java_info.to_lowercase().contains("version") {
            has_java = true;
       } else if java_info.to_lowercase().contains("jdk") {
           has_java = true;
       } else if java_info.to_lowercase().contains("runtime enviroment") {
           has_java = true;
       } else if java_info.to_lowercase().contains("64-bit") {
           has_java = true;
       } else {
           has_java = false;
       }

       if has_java == true {
        println!("Java was found!");
       } else {
           println!("Java wasn't found or is missing!");
       }
        return ("win".to_string(), has_java);
            } else {
                let output = Command::new("java")
                    .args(&["-version"])
                    .output()
                    .expect("Failed to check for Java");
                let java_info = String::from_utf8_lossy(&output.stderr);       
                if java_info.to_lowercase().contains("version") {
                    has_java = true;
                } else if java_info.to_lowercase().contains("jdk") {
                     has_java = true;
                } else if java_info.to_lowercase().contains("runtime enviroment") {
                    has_java = true;
                } else if java_info.to_lowercase().contains("64-bit") {
                    has_java = true;
                } else {
                    has_java = false;
                }
                if has_java == true {
                    println!("Java was found!");
                } else {
                    println!("Java wasn't found or is missing!");
                }
                return ("unix".to_string(), has_java); 
            }


    }

fn check_java_silent() -> bool{ 
    let mut os_name = read_cfg_silent();
    let mut has_java = false;
    while os_name == "rerun" {
            os_name = read_cfg_silent();
    }
    if os_name.contains("Windows") {
        let output = Command::new("java")
            .args(&["-version"])
            .output()
            .expect("Failed to check for Java");
        let java_info = String::from_utf8_lossy(&output.stderr);       
       if java_info.to_lowercase().contains("version") {
            has_java = true;
       } else if java_info.to_lowercase().contains("jdk") {
           has_java = true;
       } else if java_info.to_lowercase().contains("runtime enviroment") {
           has_java = true;
       } else if java_info.to_lowercase().contains("64-bit") {
           has_java = true;
       } else {
           has_java = false;
       }
        return has_java;
            } else {
                let output = Command::new("java")
                    .args(&["-version"])
                    .output()
                    .expect("Failed to check for Java");
                let java_info = String::from_utf8_lossy(&output.stderr);       
                if java_info.to_lowercase().contains("version") {
                    has_java = true;
                } else if java_info.to_lowercase().contains("jdk") {
                     has_java = true;
                } else if java_info.to_lowercase().contains("runtime enviroment") {
                    has_java = true;
                } else if java_info.to_lowercase().contains("64-bit") {
                    has_java = true;
                } else {
                    has_java = false;
                }
                return has_java; 
            }


    }

fn start_manual() {
    let mut pathsearch = true;

    println!("Please enter the path to your server.jar");
    println!("Type abort to exit.");

    while pathsearch {
        print!("-> ");
        io::stdout().flush().unwrap();

        let mut path_to_jar = String::new();
        io::stdin()
            .read_line(&mut path_to_jar)
            .expect("Could not read the Input");

        let path_to_jar = path_to_jar.trim();

        if path_to_jar.eq_ignore_ascii_case("abort") {
            break;
        }


        match fs::read(path_to_jar) {
            Ok(_) => {
                println!("Path is Valid.");

                let java = check_java_silent();

                pathsearch = false;

                if java {
                    let os_type = check_os();

                    if os_type.to_lowercase().contains("windows") {
                        let command_path_jar = Path::new(path_to_jar);

                        let command_path: PathBuf = match command_path_jar.parent() {
                            Some(parent) => parent.to_path_buf(),
                            None => {
                                println!("Could not determine the directory of the jar file.");
                                continue;
                            }
                        };

                        println!("Running server in directory: {}", command_path.display());

                        let output = Command::new("java")
                            .args(&[
                                "-Xmx1024M",
                                "-Xms1024M",
                                "-jar",
                                command_path_jar.to_str().unwrap(),
                                "nogui",
                            ])
                            .current_dir(&command_path)
                            .output()
                            .expect("Failed to start Server");

                        let server_log = String::from_utf8_lossy(&output.stderr);
                        println!("{}", server_log);
                    } else {
                        let command_path_jar = Path::new(path_to_jar);

                        let command_path: PathBuf = match command_path_jar.parent() {
                            Some(parent) => parent.to_path_buf(),
                            None => {
                                println!("Could not determine the directory of the jar file.");
                                continue;
                            }
                        };

                        println!("Running server in directory: {}", command_path.display());

                        let output = Command::new("java")
                            .args(&[
                                "-Xmx1024M",
                                "-Xms1024M",
                                "-jar",
                                command_path_jar.to_str().unwrap(),
                                "nogui",
                            ])
                            .current_dir(&command_path)
                            .output()
                            .expect("Failed to start Server");

                        let server_log = String::from_utf8_lossy(&output.stderr);
                        println!("{}", server_log);
                    }
                } else {
                    println!("Java wasn't found or is missing!");
                }
            }
            Err(_) => {
                println!("Path does not lead to a valid .jar File");
            }
        }
    }
}