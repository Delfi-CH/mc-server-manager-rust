// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::io::{self, Read, Write};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use dir::home_dir;
use std::process::exit;

//Structs

//Structs for config file

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    title: String,
    system: System,
    storage: Storage,
    #[serde(default)]
    server_list: Servers,
}

#[derive(Serialize, Deserialize, Debug)]
struct System {
    os: String,
    os_mini: String,
    servers: i32,
    after_initial_setup: bool,
    data_path: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Storage {
    use_default_server_dir: bool,
    directory: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Servers {
    #[serde(default)]
    server_list: HashMap<String, String>,
}
impl Default for Servers {
    fn default() -> Self {
        Servers {
            server_list: HashMap::new(),
        }
    }
}


// Structs for a server config file
#[derive(Serialize, Deserialize, Debug)]
struct ServerConfigFile {
    title: String,
    server_config: ServerConfigData,
}
#[derive(Serialize, Deserialize, Debug)]
struct ServerConfigData {
    name: String,
    version: String,
    modloader: String,
    path_windows_dir: String,
    path_unix_dir: String,
    path_windows_jar: String,
    path_unix_jar: String,   
    min_mem: i32,
    max_mem: i32,
    eula: bool,
    port: i32,
}


fn main() {

    init_silent();

    println!("Welcome to the CLI MC-Server Management");
    println!("What would you like to do?");
    println!();
    println!("Actions: ");
    println!("abort: Exits the Application");
    println!("about: Shows Information about the Application");
    println!("add: Adds a Server via a TOML File");
    println!("check: Checks if Java is installed on the System");
	println!("exit: Exits the Application");
	println!("init: Looks for a config.toml file. If this file isnt found, it creats it");
	println!("install: Download and install a server.jar from the Internet");
	println!("help: Lists all Actions");
    println!("license: Shows all Information about licensing.");
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
            "about" => {
                println!();
                println!("mc-server-manager");
                println!();
                println!("Developed by:");
                print!("Delfi-CH - ");
                println!("https://delfi.dev/");
                println!();
                println!("This program is licensed under the terms of the GNU General Public License Version 3.");
                println!("For more information, run the license action.");
                println!();
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
                println!("about: Shows Information about the Application");
				println!("add: Adds a Server via a TOML File");
				println!("check: Checks if Java is installed on the System");
				println!("exit: Exits the Application");
				println!("init: Looks for a config.toml file. If this file isnt found, it creats it");
				println!("install: Download and install a Server from the Internet");
				println!("help: Lists all Actions");
                println!("license: Shows all Information about licensing.");
				println!("newcfg: Generates a new config.toml");
				println!("readcfg: Reads the current config.toml and prints them");
				println!("source: Opens the projects Git Repository in your default Browser");
				println!("start: Start a Server");
				println!("startjar: Start a Server from a .jar file");
            }
            "start" => {
                start_toml();
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
            "license" => {
                println!();
                println!("This program is licensed under the terms of the GNU General Public License Version 3 (GPLv3).");
                println!("For more information, please visit https://www.gnu.org/licenses/gpl-3.0");
                println!("However, this program can download and execute the propritary licensed Minecraft server.jar via seperate processes.");
                println!("These functions require agreeing to the Minecraft End User License Agreement (EULA).");
                println!("For more information, please visit https://www.minecraft.net/en-us/eula.");
                println!();
            }
            _ => {
                println!("'{}' is not a valid Action", input);
            }
        }
    }
}

fn add_server() {
    loop {
        println!("Enter file path:");
        println!("Type abort to exit.");
        print!("-> ");
            io::stdout().flush().expect("Failed to flush stdout");

        let mut input_path = String::new();
        io::stdin()
        .read_line(&mut input_path)
        .expect("Failed to read path");

        if input_path == "abort" {
            break;
        }

        let path = input_path.trim();
        let full_path = mk_path_absolute(path);

        let filetype = Path::new(&full_path).extension().and_then(|ext| ext.to_str());

        if filetype == Some("toml") {
            match fs::read_to_string(&full_path) {
                Ok(contents_string) => {
                    println!("File is a toml file");

                    if !contents_string.contains("[server_config]") {
                        println!("File is not a valid Server configuration file!");
                        println!("Try again");
                        continue;
                    }

                    let cfg_data_str = read_cfg_silent();
                    let mut cfg_data_toml: Config = match toml::from_str(&cfg_data_str) {
                        Ok(cfg) => cfg,
                        Err(e) => {
                            eprintln!("Could not parse config.toml: {}", e);
                            continue;
                        }
                    };

                    let server_toml_str = match fs::read_to_string(&full_path) {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Failed to read server config file: {}", e);
                            continue;
                        }
                    };

                    let server_toml_toml: ServerConfigFile = match toml::from_str(&server_toml_str) {
                        Ok(srv) => srv,
                        Err(e) => {
                            eprintln!("Could not parse server config TOML: {}", e);
                            continue;
                        }
                    };

                    let is_windows = cfg_data_toml.system.os_mini == "win";

                    let jar_path = if is_windows {
                        &server_toml_toml.server_config.path_windows_jar
                    } else {
                        &server_toml_toml.server_config.path_unix_jar
                    };

                    if let Err(_) = fs::metadata(jar_path) {
                        println!("No server.jar found at the specified path: {}", jar_path);
                        continue;
                    }

                    let mut server_list = cfg_data_toml.server_list.server_list.clone();

                    let mut server_count = cfg_data_toml.system.servers;
                    server_count += 1;

                    let key = format!("server{}", server_count);
                    let path_str = full_path.display().to_string();
                    let clean_path = path_str.strip_prefix(r"\\?\").unwrap_or(&path_str);
                    server_list.insert(key, clean_path.to_string());

                    cfg_data_toml.system.servers = server_count;
                    cfg_data_toml.server_list.server_list = server_list;

                    write_cfg(&cfg_data_toml, "config.toml");
                    println!("Server added successfully.");

                    break;
                }
                Err(e) => {
                    println!("Failed to read file: {}", e);
                    continue;
                }
            }
        } else {
            println!("File is not TOML! Please enter a path to a TOML file.");
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
    init_setup(false);
}

fn init_setup(is_cfg_regenerated: bool) {
    let cfg_data_str = read_cfg_silent();
    let mut cfg_data_toml: Config = toml::from_str(&cfg_data_str)
    .expect("Could not parse TOML");
    if cfg_data_toml.system.after_initial_setup == false {

        //Set flags for initial setup
        let mut after_inital_setup = false;
        let mut server_dir_set = false;

        if is_cfg_regenerated == false {
        println!("Welcome to the CLI MC-Server Management");
        println!("Since this is the first time running the Application, we need to do some configuration.");

        let mut data_dir: PathBuf = home_dir().expect("Could not find home directory");
        data_dir.push(".mc-server-manager");
        cfg_data_toml.system.data_path = data_dir.to_string_lossy().to_string();
        write_cfg(&cfg_data_toml, "config.toml");
        
        println!("Creating data directory...");
        match fs::metadata(&data_dir) {
            Ok(_) => {
            println!("Directory already exists!");  
            }
            Err(_) => {
                fs::create_dir(data_dir).expect("Could not create directory");
            }
        }
        
        

        } else {
            println!("After regenerating the configuration file, you need to set some configuration.");
        }
        println!("Do you want to use the default directory for storing servers?");
        println!("This is either C:\\Users\\[your username]\\.mc-server-manager\\servers on Windows or /home/[your username]/.mc-server-manager/servers on Linux");
        println!("y/n");
        print!("-> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read the Input");

        let input= input.to_lowercase();
        let input = input.trim();

        println!("{}", input);

        if input == "y" {

            let mut default_server_dir: PathBuf = home_dir().expect("Could not find home directory");
            default_server_dir.push(".mc-server-manager/servers");
            println!("Setting server directory to the default Value ({})", default_server_dir.to_string_lossy());
            cfg_data_toml.storage.use_default_server_dir = true;
            cfg_data_toml.storage.directory = default_server_dir.to_string_lossy().to_string();
            write_cfg(&cfg_data_toml, "config.toml");
            server_dir_set = true;
            match fs::metadata(&default_server_dir) {
            Ok(_) => {
            println!("Directory already exists!");  
            }
            Err(_) => {
                fs::create_dir(default_server_dir).expect("Could not create directory");
            }
        }

        } else if input == "n" {

        let mut is_path_set = false;

        while is_path_set == false {
            

        println!("Please enter the directory where the servers will be stored.");
        print!("-> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read the Input");

        let input= input.to_lowercase();
        let input = input.trim();

        match fs::metadata(input) {
        Ok(_) => {
            println!("Setting server directory to {}.", input);
            cfg_data_toml.storage.use_default_server_dir = false;
            cfg_data_toml.storage.directory = input.to_string();
            write_cfg(&cfg_data_toml, "config.toml");
            is_path_set = true;
            server_dir_set = true;
        }
        Err(_) => {
            println!("{} is not a valid directory!", input);
        } 
    }
    }
        } else {
            println!("Not a valid Input!");
        }
    
    if server_dir_set == true {
        after_inital_setup = true;
    }

    if after_inital_setup == true {
        cfg_data_toml.system.after_initial_setup = true;
        write_cfg(&cfg_data_toml, "config.toml");
        println!("Initial Setup Complete!");
    }

    } else {
        return;
    }
}

fn init_silent() {
    match fs::read("config.toml") {
        Ok(_) => {
            init_setup(false);
        }
        Err(_) => {
            new_cfg_silent();
            init_setup(false);
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
            new_cfg_silent();
            println!("Finished!");
        }
        Err(_) => {
            println!("config.toml wasn't found, creating it...");
            new_cfg_silent();
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
                .write_all("title = \"mc-server-manager_config\"\n\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("[system]\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("os = \"{}\" \n", check_os().trim()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("os_mini = \"{}\" \n", check_os_mini().trim()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("servers = 0\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("after_initial_setup = false\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("data_path = \"\"\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("[storage]\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("use_default_server_dir = false\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("directory = \"none\"\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("[servers]\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("server_list = {}\n".as_bytes())
                .expect("Could not write to file");
        }
        Err(_) => {
            let mut cfg_file = File::create("config.toml").expect("Could not create file");
            cfg_file
                .write_all("# Config for mc-server-management\n\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("title = \"mc-server-manager_config\"\n\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("[system]\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("os = \"{}\" \n", check_os().trim()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("os_mini = \"{}\" \n", check_os_mini().trim()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("servers = 0\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("after_initial_setup = false\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("data_path = \"\"\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("[storage]\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("use_default_server_dir = false\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("directory = \"none\"\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("[servers]\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("server_list = {}\n".as_bytes())
                .expect("Could not write to file");
        }
    }
    init_setup(true);
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

fn write_cfg(config: &Config, path: &str) {
    let toml_string = toml::to_string_pretty(config)
        .expect("Failed to serialize config to TOML");
    fs::write(path, toml_string)
        .expect("Failed to write config file");
}

fn check_os() -> String {
    let info = os_info::get();
    let os_info = format!("{}", info);
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

    let os_info_mini = format!("{}", os_mini);
    os_info_mini
}

fn check_java() -> bool {
    let mut os_name = read_cfg_silent();
    while os_name == "rerun" {
        os_name = read_cfg_silent();
    }

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

    has_java
}

fn check_java_silent() -> bool{ 
    let mut os_name = read_cfg_silent();
    let has_java:bool;
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
            .expect("Could not read the input");

        let path_to_jar = path_to_jar.trim();

        if path_to_jar.eq_ignore_ascii_case("abort") {
            break;
        }

        match fs::read(path_to_jar) {
            Ok(_) => {
                if !check_java_silent() {
                    println!("Java wasn't found or is missing!");
                    continue;
                }

                let command_path_jar = Path::new(path_to_jar);
                let command_path = match command_path_jar.parent() {
                    Some(parent) => parent.to_path_buf(),
                    None => {
                        println!("Could not determine the directory of the jar file.");
                        continue;
                    }
                };

                let eula_path = command_path.join("eula.txt");
                let mut agree_eula = false;

                if let Ok(contents) = fs::read_to_string(&eula_path) {
                    if contents.contains("eula = true") {
                        agree_eula = true;
                    }
                }

                while !agree_eula {
                    println!("Do you agree to the Minecraft EULA?");
                    println!("https://www.minecraft.net/en-us/eula");
                    println!("y/n/open");
                    print!("-> ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().to_lowercase();

                    match input.as_str() {
                        "y" => agree_eula = true,
                        "n" => break,
                        "open" => {
                            if let Err(e) = open::that("https://www.minecraft.net/en-us/eula") {
                                eprintln!("Failed to open browser: {}", e);
                            }
                        }
                        _ => println!("Not a valid input"),
                    }
                }

                if !agree_eula {
                    break;
                }

                // Write agreement
                let _ = fs::write(&eula_path, "eula = true");

                println!("Set the minimum amount of RAM for the Server in MB");
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

                println!(
                    "Running: java -Xms{}M -Xmx{}M -jar {} nogui",
                    min_mem_int,
                    max_mem_int,
                    path_to_jar
                );

                start_generic(command_path_jar, &command_path, min_mem_int, max_mem_int, agree_eula);
                
                pathsearch = false;
            }
            Err(_) => {
                println!("Path does not lead to a valid .jar file.");
            }
        }
    }
}

fn start_generic(jar_path: &Path, command_path: &Path, mem_min: u32, mem_max: u32, eula: bool) -> bool {
if eula == true {
    let xms_arg = format!("-Xms{}M", mem_min);
    let xmx_arg = format!("-Xmx{}M", mem_max);

    Command::new("java")
        .args([
        xms_arg,
        xmx_arg,
        "-jar".to_string(),
        jar_path.to_str().unwrap().to_string(),
        "nogui".to_string(),
        ])
        .current_dir(&command_path)
        .output()
        .expect("Failed to start Server");

    return true;
} else {return false;}
}

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
        Command::new("curl")
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

fn start_toml() {
    let cfg_app_str = read_cfg_silent();
    let cfg_app_data: Config = toml::from_str(&cfg_app_str)
        .expect("Could not parse TOML");

    let mut has_selected_server = false;

    if cfg_app_data.system.servers != 0 {

    println!("List of servers:");
    println!();

    for (server_name, filename) in &cfg_app_data.server_list.server_list {
        println!("{} => {}", server_name, filename);
    }

    println!();

    println!("What server do you want to start?");
    println!("Please enter a number.");
    println!("or type abort to exit.");

    let mut input1 = String::new();

    while has_selected_server == false {
        print!("-> ");
        io::stdout().flush().unwrap();

        input1.clear();
        io::stdin()
            .read_line(&mut input1)
            .expect("Could not read the Input");

        input1 = input1.trim().to_lowercase();

        if input1 == "abort" {
            println!("Exiting starting process...");
            return;
        } else if input1.chars().any(|c| c.is_digit(10)) {
            has_selected_server = true;
            break;
        } else {
            println!("Please enter a Number!");
        }
    }

    if has_selected_server == true {
        let mut server_name = String::from("server");
        server_name += &input1;
        println!("Selected {}", server_name);

        if cfg_app_data.server_list.server_list.contains_key(&server_name) == true {
            let server_toml_path = cfg_app_data.server_list.server_list["server1"].clone();
            println!("{}", server_toml_path);

            let cfg_server_str = fs::read_to_string(server_toml_path)
            .expect("Could not read file");
            let cfg_server_toml : ServerConfigFile =  toml::from_str(&cfg_server_str).expect("Could not parse TOML"); 

            #[warn(unused_mut)]
            //is only here bc warn is kind of buggy
            let mut path_jar_str = String::new();
            if cfg_app_data.system.os_mini == "win" {
                path_jar_str = cfg_server_toml.server_config.path_windows_jar;
            } else {
                path_jar_str = cfg_server_toml.server_config.path_unix_jar;
            }
            #[warn(unused_mut)]
            //is only here bc warn is kind of buggy
            let mut path_dir_str = String::new();
            if cfg_app_data.system.os_mini == "win" {
                path_dir_str = cfg_server_toml.server_config.path_windows_dir;
            } else {
                path_dir_str = cfg_server_toml.server_config.path_unix_dir;
            }
            let mut agree_eula = cfg_server_toml.server_config.eula;
            
            let eula_path = path_dir_str.clone() + "eula.txt";

            if let Ok(contents) = fs::read_to_string(&eula_path) {
                    if contents.contains("eula = true") {
                        agree_eula = true;
                    }
                }

            if agree_eula == false { 
                
                println!("Do you agree to the Minecraft EULA?");
                println!("https://www.minecraft.net/en-us/eula");
                println!("y/n/open");

                while !agree_eula {
                    print!("-> ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().to_lowercase();

                    match input.as_str() {
                        "y" => agree_eula = true,
                        "n" => break,
                        "open" => {
                            if let Err(e) = open::that("https://www.minecraft.net/en-us/eula") {
                                eprintln!("Failed to open browser: {}", e);
                            }
                        }
                        _ => println!("Not a valid input"),
                    }
                }
                let _ = fs::write(&eula_path, "eula = true");
                }
            
            let mem_min:u32 = cfg_server_toml.server_config.min_mem.try_into().unwrap();
            let mem_max:u32 = cfg_server_toml.server_config.max_mem.try_into().unwrap();
            
            let path_to_jar = Path::new(&path_jar_str);
            let path_server_dir = Path::new(&path_dir_str);

            println!("Starting Server...");
            
            start_generic(path_to_jar, path_server_dir, mem_min, mem_max, agree_eula);
        }
} else {
    println!("No Server found!");
    println!("Please add a Server via the add action.");
}
}
}

fn mk_path_absolute(input_path: &str) -> PathBuf {
    let path = Path::new(input_path);

    if path.is_absolute() {
        std::fs::canonicalize(path).expect("Failed to canonicalize absolute path")
    } else {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        std::fs::canonicalize(current_dir.join(path)).expect("Failed to canonicalize joined path")
    }
}
