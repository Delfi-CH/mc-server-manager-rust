//use std::fmt::Error;
use std::io::{self, Read, Write};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
//use json::stringify;
use dir::home_dir;
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
    println!("init: Looks for a config.toml file. If this file isnt found, it creats it");
    println!("install: Download and install a Server from the Internet");
    println!("help: Lists all Actions");
    println!("newcfg: Generates a new config.toml");
    println!("readcfg: Reads the current config.toml and prints them");
    println!("source: Opens the projects Git Repository in your default Browser");
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
                download_server();
            }
            "help" => {
				println!("Actions: ");
				println!("abort: Exits the Application");
				println!("add: Adds a Server via a JSON File");
				println!("check: Checks if Java is installed on the System");
				println!("exit: Exits the Application");
				println!("init: Looks for a config.toml file. If this file isnt found, it creats it");
				println!("install: Download and install a Server from the Internet");
				println!("help: Lists all Actions");
				println!("newcfg: Generates a new config.toml");
				println!("readcfg: Reads the current config.toml and prints them");
				println!("source: Opens the projects Git Repository in your default Browser");
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

fn init() {
    match fs::read("config.toml") {
        Ok(_) => {
            println!("Found config.toml");
        }
        Err(_) => {
            new_cfg();
        }
    }
}

fn new_cfg(){
    match fs::read("config.toml") {
        Ok(_) => {
            println!("Found config.toml");
            println!("Removing config.toml...");
            fs::remove_file("config.toml").expect("Could not delete file");
            println!("Creating new config.toml...");
            let mut cfg_file = File::create("config.toml").expect("Could not create file");
            cfg_file
                .write_all("# Config for mc-server-management\n\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("[system]\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("os = \"{}\"\n", check_os()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("os_mini = \"{}\"\n", check_os_mini()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("servers = 0\n".as_bytes())
                .expect("Could not write to file");
            println!("Finished!");
        }
        Err(_) => {
            println!("config.toml wasn't found, creating it...");
            let mut cfg_file = File::create("config.toml").expect("Could not create file");
            cfg_file
                .write_all("# Config for mc-server-management\n\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("[system]\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("os = \"{}\"\n", check_os()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("os_mini = \"{}\"\n", check_os_mini()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("servers = 0\n".as_bytes())
                .expect("Could not write to file");
            println!("Finished!");
        }
    }
}

fn new_cfg_silent(){
    // same as fn new_cfg, but doesnt print output
    match fs::read("config.toml") {
        Ok(_) => {
            fs::remove_file("config.toml").expect("Could not delete file");
            let mut cfg_file = File::create("config.toml").expect("Could not create file");
            cfg_file
                .write_all("# Config for mc-server-management\n\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("[system]\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("os = \"{}\"\n", check_os()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("os_mini = \"{}\"\n", check_os_mini()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("servers = 0\n".as_bytes())
                .expect("Could not write to file");
        }
        Err(_) => {
            let mut cfg_file = File::create("config.toml").expect("Could not create file");
            cfg_file
                .write_all("# Config for mc-server-management\n\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("[system]\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("os = \"{}\"\n", check_os()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("os_mini = \"{}\"\n", check_os_mini()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("servers = 0\n".as_bytes())
                .expect("Could not write to file");
        }
    }
}

fn read_cfg() {
    match File::open("config.toml") {
        Ok(mut app_cfg) => {
            let mut app_cfg_content = String::new();
            if let Err(e) = app_cfg.read_to_string(&mut app_cfg_content) {
                eprintln!("Error reading file: {}", e);
                return;
            }
            println!();
            println!("Contents of config.toml:");
            println!("-----------------------");
            println!("{}", app_cfg_content);
            println!("-----------------------");
            println!();
        }
        Err(_) => {
            println!("config.toml not found!");
            println!();
            println!("Would you like to generate a new config.toml? (y/n)");
            print!("-> ");
            io::stdout().flush().unwrap();

            let mut read_cfg_yn = String::new();

            io::stdin()
            .read_line(&mut read_cfg_yn)
            .expect("Could not read the Input");

            let read_cfg_yn= read_cfg_yn.to_lowercase();
            let read_cfg_yn = read_cfg_yn.trim();
            
            if read_cfg_yn == "y" {
                new_cfg();
            } else {
                println!("Aborting...");
            }
        }
    }
}
fn read_cfg_silent() -> String {

    // same as fn read_cfg, but doesnt print output
    match File::open("config.toml") {
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
            new_cfg_silent();
            let return_error_statement = "rerun";
            return return_error_statement.to_string();        
        }
    }
}

fn check_os() -> String {
    let info = os_info::get();
    let os_info = format!("OS: {}\n", info);
    os_info
}

fn check_os_mini() -> String {
    //Works the same as check_os, but writes a short name (e.g. win, linux)
    let info = os_info::get();


    let os_mini = if info.to_string().contains("Windows") {
        "win"
    } else {
        "unix"
    };

    let os_info_mini = format!("OS-Mini: {}\n", os_mini);
    os_info_mini
}

fn check_java() -> (String, bool) {
    let mut os_name = read_cfg_silent();
    while os_name == "rerun" {
        os_name = read_cfg_silent();
    }

    let platform = if os_name.contains("Windows") {
        "win"
    } else {
        "unix"
    };

    let output = Command::new("java")
        .args(&["-version"])
        .output();

    let has_java = match output {
        Ok(output) => {
            let java_info = String::from_utf8_lossy(&output.stderr).to_lowercase();
            java_info.contains("version") ||
            java_info.contains("jdk") ||
            java_info.contains("runtime environment") ||
            java_info.contains("64-bit")
        }
        Err(_) => false,
    };

    if has_java {
        println!("Java was found!");
    } else {
        println!("Java wasn't found or is missing!");
    }

    (platform.to_string(), has_java)
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
                        //Does all the EULA Stuff
                        let mut agree_eula = false;

                        let eula_path_buf = {
                        let eula_path = Path::new(path_to_jar);
                        match eula_path.parent() {
                            Some(parent) => parent.join("eula.txt"),
                            None => {
                         println!("Could not find EULA.txt.");
                        continue;
                        }
                    }
                    };

                    let eula_contents = fs::read_to_string(&eula_path_buf)
                        .expect("Should have been able to read the file");
                    if eula_contents.contains("eula = true") {
                        agree_eula = true;
                    } else {
                        while agree_eula == false {
                        println!("Do you agree to the Minecraft EULA?");
                        println!("https://www.minecraft.net/en-us/eula");
                        println!("y/n/show");
                        print!("-> ");
                        io::stdout().flush().unwrap();

                        
                        let mut agree_eula_input = String::new();

                        io::stdin()
                            .read_line(&mut agree_eula_input)
                            .expect("Could not read the Input");
                        let agree_eula_input = agree_eula_input.trim().to_lowercase();
                        
                        if agree_eula_input == "y" {
                            agree_eula = true;
                        } else if agree_eula_input == "n" {
                            break;
                        } else if agree_eula_input == "open" {
                            if let Err(e) = open::that("https://www.minecraft.net/en-us/eula") {
                            eprintln!("Failed to open browser: {}", e);
                            }
                        } else {
                            println!("Not a valid Input");
                        }
                    }
                }
                    if agree_eula == true {

                        let eula_path = command_path.join("eula.txt");

                        let _eula = fs::write(eula_path, "eula = true");

                        println!("Running server in directory: {}", command_path.display());

                        //Add Maximum and Minimum Mem here

                        println!("Set the minimum mmount of RAM for the Server in MB");
                        print!("-> ");
                        io::stdout().flush().unwrap();


                        let mut min_mem_input = String::new();
                        io::stdin().read_line(&mut min_mem_input).unwrap();
                        let min_mem_int: u32 = min_mem_input.trim().parse().expect("Must be a number");

                        println!("Set the maximum amount of RAM for the Server in MB");
                        print!("-> ");
                        io::stdout().flush().unwrap();

                        let mut max_mem_input = String::new();
                        io::stdin().read_line(&mut max_mem_input).unwrap();
                        let max_mem_int: u32 = max_mem_input.trim().parse().expect("Must be a number");

                        let xms_arg = format!("-Xms{}M", min_mem_int);
                        let xmx_arg = format!("-Xmx{}M", max_mem_int);
                        


                            let output = Command::new("java")
                            .args(&[
                                &xms_arg,
                                &xmx_arg,
                                "-jar",
                                command_path_jar.to_str().unwrap(),
                                "nogui",
                            ])
                            .current_dir(&command_path)
                            .output()
                            .expect("Failed to start Server");

                        println!("Running: java {} {} -jar {} nogui", xmx_arg, xms_arg, path_to_jar);

                        let server_log = String::from_utf8_lossy(&output.stderr);
                        println!("{}", server_log);
                            } else {
                                break;
                            }
                    } else {
                        let mut agree_eula = false;

                        let eula_path_buf = {
                        let eula_path = Path::new(path_to_jar);
                        match eula_path.parent() {
                            Some(parent) => parent.join("eula.txt"),
                            None => {
                         println!("Could not find EULA.txt.");
                        continue;
                        }
                    }
                    };

                    let eula_contents = fs::read_to_string(&eula_path_buf)
                        .expect("Should have been able to read the file");
                    if eula_contents.contains("eula = true") {
                        agree_eula = true;
                    } else {
                        while agree_eula == false {
                        println!("Do you agree to the Minecraft EULA?");
                        println!("https://www.minecraft.net/en-us/eula");
                        println!("y/n/show");
                        print!("-> ");
                        io::stdout().flush().unwrap();

                        
                        let mut agree_eula_input = String::new();

                        io::stdin()
                            .read_line(&mut agree_eula_input)
                            .expect("Could not read the Input");
                        let agree_eula_input = agree_eula_input.trim().to_lowercase();
                        
                        if agree_eula_input == "y" {
                            agree_eula = true;
                        } else if agree_eula_input == "n" {
                            break;
                        } else if agree_eula_input == "open" {
                            if let Err(e) = open::that("https://www.minecraft.net/en-us/eula") {
                            eprintln!("Failed to open browser: {}", e);
                            }
                        } else {
                            println!("Not a valid Input");
                        }
                    }
                }
                        if agree_eula == true {
                        let command_path_jar = Path::new(path_to_jar);

                        let command_path: PathBuf = match command_path_jar.parent() {
                            Some(parent) => parent.to_path_buf(),
                            None => {
                                println!("Could not determine the directory of the jar file.");
                                continue;
                            }
                        };
                        
                        let eula_path = command_path.join("eula.txt");

                        let _eula = fs::write(eula_path, "eula = true");

                        println!("Running server in directory: {}", command_path.display());

                        //Add Maximum and Minimum Mem here

                        println!("Set the minimum mmount of RAM for the Server in MB");
                        print!("-> ");
                        io::stdout().flush().unwrap();


                        let mut min_mem_input = String::new();
                        io::stdin().read_line(&mut min_mem_input).unwrap();
                        let min_mem_int: u32 = min_mem_input.trim().parse().expect("Must be a number");

                        println!("Set the maximum amount of RAM for the Server in MB");
                        print!("-> ");
                        io::stdout().flush().unwrap();

                        let mut max_mem_input = String::new();
                        io::stdin().read_line(&mut max_mem_input).unwrap();
                        let max_mem_int: u32 = max_mem_input.trim().parse().expect("Must be a number");

                        let xms_arg = format!("-Xms{}M", min_mem_int);
                        let xmx_arg = format!("-Xmx{}M", max_mem_int);
                        


                            let output = Command::new("java")
                            .args(&[
                                &xms_arg,
                                &xmx_arg,
                                "-jar",
                                command_path_jar.to_str().unwrap(),
                                "nogui",
                            ])
                            .current_dir(&command_path)
                            .output()
                            .expect("Failed to start Server");

                        println!("Running: java {} {} -jar {} nogui", xmx_arg, xms_arg, path_to_jar);

                        let server_log = String::from_utf8_lossy(&output.stderr);
                        println!("{}", server_log);
                        } else {
                            break;
                        }
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

/*fn start_generic(path: &Path, mem_min: i32, mem_max: i32, eula: bool) -> bool {
    //TODO: EVERYTHING    
    return true;
}*/

fn download_server() {
    let mut agree_eula = false;
    println!("Do you want to download the official Minecraft server.jar from https://www.minecraft.net/en-us/download/server?");
    println!("Note, by downloading the server.jar, you automaticly agree to the Minecraft EULA");
    while agree_eula == false {
        println!("Do you agree to the Minecraft EULA?");
        println!("https://www.minecraft.net/en-us/eula");
        println!("y/n/show");
        print!("-> ");
        io::stdout().flush().unwrap();

                        
        let mut agree_eula_input = String::new();

        io::stdin()
        .read_line(&mut agree_eula_input)
        .expect("Could not read the Input");
        let agree_eula_input = agree_eula_input.trim().to_lowercase();
                        
        if agree_eula_input == "y" {
            agree_eula = true;
        } else if agree_eula_input == "n" {
            break;
        } else if agree_eula_input == "open" {
            if let Err(e) = open::that("https://www.minecraft.net/en-us/eula") {
            eprintln!("Failed to open browser: {}", e);
            }
        } else {
        println!("Not a valid Input");
        }
        }

    if agree_eula == true{
        let mut download_path: PathBuf = home_dir().expect("Could not find home directory");
        download_path.push("Downloads/server.jar");
        println!("Downloading server.jar ...");
        //needs testing on linux
        let output = Command::new("curl")
        .args(&[
        "https://piston-data.mojang.com/v1/objects/e6ec2f64e6080b9b5d9b471b291c33cc7f509733/server.jar",
        "-o",
        download_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to download File");
        println!("Finished!");
    }
        }