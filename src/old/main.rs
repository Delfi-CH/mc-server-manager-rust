// SPDX-License-Identifier: GPL-3.0-or-later

//Imports
use serde::{Deserialize, Serialize};
use std::collections::{HashMap};
use indexmap::IndexMap;
use std::env::{self};
use std::io::{self, Read, Write, BufWriter, BufReader, BufRead};
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use dir::{home_dir};
use std::process::exit;
use std::{thread, time::Duration};
use std::process::{Child, ChildStdin, ChildStdout, ChildStderr};
use props_rs::*;
#[cfg(unix)]
use libc;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

use once_cell::sync::Lazy;
use std::sync::Mutex;


//Consts

//Used for spawning java on Windows
#[warn(dead_code)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

const MIN_MEM_DEFAULT: i32 = 512;
const MAX_MEM_DEFAULT: i32 = 2048;
const PORT_DEFAULT: i32 = 25565;

//Statics

static SERVER_REGISTRY: Lazy<Mutex<HashMap<String, ServerHandle>>> = Lazy::new(|| Mutex::new(HashMap::new()));

//Structs

//Structs for spawning and reading stdin/stout on server

pub struct ServerHandle {
    pub process: Child,
}

impl ServerHandle {
    pub fn stdin(&mut self) -> Option<&mut ChildStdin> {
        self.process.stdin.as_mut()
    }

    pub fn stdout(&mut self) -> Option<&mut ChildStdout> {
        self.process.stdout.as_mut()
    }

    pub fn stderr(&mut self) -> Option<&mut ChildStderr> {
        self.process.stderr.as_mut()
    }

    pub fn pid(&self) -> u32 {
        self.process.id()
    }
}

//Structs for config file

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    title: String,
    #[serde(default)]
    system: System,
    #[serde(default)]
    mcsvdl: McsvdlInfo,
    #[serde(default)]
    storage: Storage,
    #[serde(default)]
    server_list: Servers,
}

#[derive(Serialize, Deserialize, Debug)]
struct System {
    #[serde(default)]
    os: String,
    #[serde(default)]
    os_mini: String,
    #[serde(default)]
    servers: i32,
    #[serde(default)]
    after_initial_setup: bool,
    #[serde(default)]
    data_path: String,
}

impl Default for System {
    fn default() -> Self {
        System {
            os: String::new(),
            os_mini: String::new(),
            servers: 0,
            after_initial_setup: false,
            data_path: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct McsvdlInfo {
    #[serde(default)]
    has_mcsvdl: bool,
    #[serde(default)]
    mcsvdl_path: String,
    #[serde(default)]
    mcsvdl_version: String,
}

impl Default for McsvdlInfo {
    fn default() -> Self {
        McsvdlInfo {
            has_mcsvdl: false,
            mcsvdl_path: String::new(),
            mcsvdl_version: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Storage {
    #[serde(default)]
    use_default_server_dir: bool,
    #[serde(default)]
    directory: String,
}

impl Default for Storage {
    fn default() -> Self {
        Storage {
            use_default_server_dir: false,
            directory: "none".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Servers {
    #[serde(default)]
    server_list: IndexMap<String, String>,
}

impl Default for Servers {
    fn default() -> Self {
        Servers {
            server_list: IndexMap::new(),
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
    running: bool,
    pid: String,
    port: i32,
}

// Struct for getting the github release number

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
}

type FmlVersionsFile = HashMap<String, String>;

//fn main
fn main() {

    init_silent();
    get_active_servers();

    println!("Welcome to the CLI MC-Server Management");
    println!("What would you like to do?");
    println!();
	println!("Actions: ");
	println!("abort: Exits the Application");
    println!("about: Shows Information about the Application");
	println!("add: Adds a Server via a TOML File");
	println!("check: Checks if Java is installed on the System");
    println!("console: Print the Console Output of a specific server.");
	println!("exit: Exits the Application");
	println!("init: Looks for a config.toml file. If this file isnt found, it creats it");
    println!("info: Get info about a server.");
	println!("install: Download and install a Server from the Internet");
	println!("help: Lists all Actions");
    println!("license: Shows all Information about licensing.");
    println!("list: Shows all active Servers");
	println!("newcfg: Generates a new config.toml");
	println!("readcfg: Reads the current config.toml and prints them");
	println!("source: Opens the projects Git Repository in your default Browser");
	println!("start: Start a Server");
	println!("startjar: Start a Server from a .jar file");
    println!("stop: Stops a Sever started via the start action.");

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
                println!("info: Get info about a server.");
				println!("init: Looks for a config.toml file. If this file isnt found, it creats it");
				println!("install: Download and install a Server from the Internet");
				println!("help: Lists all Actions");
                println!("license: Shows all Information about licensing.");
                println!("list: Shows all active Servers");
				println!("newcfg: Generates a new config.toml");
				println!("readcfg: Reads the current config.toml and prints them");
				println!("source: Opens the projects Git Repository in your default Browser");
				println!("start: Start a Server");
				println!("startjar: Start a Server from a .jar file");
                println!("stop: Stops a Sever started via the start action.");
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
                exit(69);
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
            "list" => {
                list_servers();
            }
            "info" => {
                read_properties();
            }
            "stop" => {
                stop_server_wrapper();
            }
            "eee" => {
                list_servers_hash();
            }
            "console" => {
                io::stdout().flush().expect("Failed to flush stdout");

                let mut input_path = String::new();
                io::stdin()
                .read_line(&mut input_path)
                .expect("Failed to read path");
                if input_path == "abort" {
                    break;
                }
                let path = input_path.trim();

                if path.to_lowercase() == "abort" {
                    break;
                }
                let e = read_server_stdout(path);
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

        if path.to_lowercase() == "abort" {
            break;
        }

        add_server_silent(path);
       
}
}

fn add_server_silent(path: &str) {
        let full_path = win_path_cleaner_path(mk_path_absolute(path));

        println!("{}", full_path.display());

        let filetype = Path::new(&full_path).extension().and_then(|ext| ext.to_str());

        if filetype == Some("toml") {
            match fs::read_to_string(&full_path) {
                Ok(contents_string) => {
                    if !contents_string.contains("[server_config]") {
                        return;
                    }

                    let cfg_data_str = read_cfg_silent();
                    let mut cfg_data_toml: Config = match toml::from_str(&cfg_data_str) {
                        Ok(cfg) => cfg,
                        Err(e) => {
                            eprintln!("Could not parse config.toml: {}", e);
                            return;
                        }
                    };

                    let server_toml_str = match fs::read_to_string(&full_path) {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Failed to read server config file: {}", e);
                            return;
                        }
                    };

                    let server_toml_toml: ServerConfigFile = match toml::from_str(&server_toml_str) {
                        Ok(srv) => srv,
                        Err(e) => {
                            eprintln!("Could not parse server config TOML: {}", e);
                            return;
                        }
                    };

                    let is_windows = cfg_data_toml.system.os_mini == "win";

                    let is_forge = server_toml_toml.server_config.modloader.contains("forge");

                    

                    let jar_path = if is_windows {
                        &server_toml_toml.server_config.path_windows_jar
                    } else {
                        &server_toml_toml.server_config.path_unix_jar
                    };
                    if is_forge == false {

                    if let Err(_) = fs::metadata(jar_path) {
                        println!("Jar path does not exist: {}", jar_path);
                        return;
                    }

                    }

                    let mut server_list = cfg_data_toml.server_list.server_list.clone();

                    let mut server_count = cfg_data_toml.system.servers;
                    server_count += 1;

                    let key = format!("server{}", server_count);
                    let path_str = full_path.display().to_string();
                    let clean_path = match fs::canonicalize(&full_path) {
                            Ok(p) => p.display().to_string(),
                            Err(_) => path_str,
                    };

                    #[cfg(windows)]{                    
                    server_list.insert(key, win_path_cleaner(&clean_path).to_string());
                    }
                    #[cfg(unix)]{                    
                    server_list.insert(key, clean_path.to_string());
                    }

                    cfg_data_toml.system.servers = server_count;
                    cfg_data_toml.server_list.server_list = server_list;

                    write_cfg(&cfg_data_toml, "config.toml");

                    return;
                }
                Err(e) => {
                    println!("Failed to read file: {}", e);
                    return;
                }
            }
        } else {
            println!("File is not TOML! Please enter a path to a TOML file.");
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
                fs::create_dir(&data_dir).expect("Could not create directory");
            }
        }
        let mut mcsvdl_tar= data_dir.clone();
        #[cfg(windows)] {
        mcsvdl_tar.push("mcsvdl.exe");
        }
        #[cfg(unix)] {
        mcsvdl_tar.push("mcsvdl");
        }
        check_mcsvdl(data_dir.clone(), mcsvdl_tar.clone());
        

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

            let mut data_dir: PathBuf = home_dir().expect("Could not find home directory");
            data_dir.push(".mc-server-manager");
            cfg_data_toml.system.data_path = data_dir.to_string_lossy().to_string();

            let mut mcsvdl_tar= data_dir.clone();
            #[cfg(windows)] {
            mcsvdl_tar.push("mcsvdl.exe");
            }
            #[cfg(unix)] {
            mcsvdl_tar.push("mcsvdl");
            }
            check_mcsvdl(data_dir.clone(), mcsvdl_tar.clone());

            let mut default_server_dir: PathBuf = home_dir().expect("Could not find home directory");
            #[cfg(windows)]{
            default_server_dir.push(".mc-server-manager\\servers");
            }
            #[cfg(unix)]{
            default_server_dir.push(".mc-server-manager/servers");
            }
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
                fs::create_dir_all(default_server_dir).expect("Could not create directory");
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
            return; 
        }
        Err(_) => {
            new_cfg_silent();
        }
    }
    init_setup(false);
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
                .write_all("[mcsvdl]\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("has_mcsvdl = false\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("mcsvdl_path = \"\"\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("mcsvdl_version = \"\"\n".as_bytes())
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
fn fml_versions_str(mc_version: String, is_neoforge: bool) -> String {

    check_fml_vfile_updates(is_neoforge);

    let mut fml_file_path = String::new();

    #[cfg(windows)] {
        fml_file_path = home_dir().expect("Could not get HomeDir").display().to_string();
        fml_file_path = fml_file_path + "\\.mc-server-manager\\data";
    }
    #[cfg(unix)] {
        fml_file_path = home_dir().expect("Could not get HomeDir").display().to_string();
        fml_file_path = fml_file_path + "/.mc-server-manager/data"
    }

    if is_neoforge == true {
        #[cfg(windows)] {
        fml_file_path = fml_file_path + "\\neofml_versions.toml"
        }
        #[cfg(unix)] {
        fml_file_path = fml_file_path + "/neofml_versions.toml"
        }
    } else {
        #[cfg(windows)] {
        fml_file_path = fml_file_path + "\\fml_versions.toml"
        }
        #[cfg(unix)] {
        fml_file_path = fml_file_path + "/fml_versions.toml"
        }
    }
    loop {
    match File::open(&fml_file_path) {
        Ok(_) => {
            let fml_version_file_str = fs::read_to_string(fml_file_path).expect("Could not read File");
            let fml_version_file_toml: FmlVersionsFile = toml::from_str(&fml_version_file_str)
                .expect("Could not parse TOML");

            let version = fml_version_file_toml.get(&mc_version);

            if let Some(ver) = version {
                println!("Forge Modloader Version : {}", ver);
                return ver.clone();
            } else {
                return "An Error Occured".to_string();
            }
        }
        Err(_) => {
            fml_vfile_donwload(is_neoforge, fml_file_path.clone());
        }
    }
}
}

fn write_cfg(config: &Config, path: &str) {
    let toml_string = toml::to_string_pretty(config)
        .expect("Failed to serialize config to TOML");
    fs::write(path, toml_string)
        .expect("Failed to write config file");
}
fn write_server_toml(toml: &ServerConfigFile, path: &str) {
    let toml_string = toml::to_string_pretty(toml)
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

                let server_handle = start_generic(command_path_jar, &command_path, min_mem_int, max_mem_int, agree_eula, false);

                if let Some(handle) = server_handle {
                    let pid = handle.pid();
                    println!("Server started successfully with PID {}!", pid);
                    let mut registry = SERVER_REGISTRY.lock().unwrap();
                    registry.insert(handle.pid().to_string(), handle);
                
                pathsearch = false;
                } else {
                    println!("Failed to start Server!");
                }
            }
            Err(_) => {
                println!("Path does not lead to a valid .jar file.");
            }
        }
    }
}

pub fn start_generic(
    jar_path: &Path,
    command_path: &Path,
    mem_min: u32,
    mem_max: u32,
    eula: bool,
    is_fml: bool,
) -> Option<ServerHandle> {
    if !eula {
        return None;
    }

    let xms_arg = format!("-Xms{}M", mem_min);
    let xmx_arg = format!("-Xmx{}M", mem_max);

    let cfg_app_str = read_cfg_silent();
    let cfg_app_data: Config = toml::from_str(&cfg_app_str).expect("Could not parse TOML");

    let server: Option<Child>;

    if !is_fml {
        if cfg_app_data.system.os.to_lowercase().contains("windows") {
            #[cfg(windows)]
            {
                let process = Command::new("java")
                    .args([
                        xms_arg,
                        xmx_arg,
                        "-jar".to_string(),
                        jar_path.display().to_string(),
                        "nogui".to_string(),
                    ])
                    .current_dir(command_path)
                    .creation_flags(0x08000000) // CREATE_NO_WINDOW
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("Failed to start Java process");

                thread::sleep(Duration::from_secs(5));

                let jps = Command::new("jps").arg("-l").output().expect("Failed to run jps");
                let jps_str = String::from_utf8_lossy(&jps.stdout).to_lowercase();

                if jps_str.contains(&command_path.to_string_lossy().to_lowercase()) {
                    return Some(ServerHandle { process });
                }
            }
        } else if cfg_app_data.system.os_mini.to_lowercase().contains("unix") {
            #[cfg(unix)]
            unsafe {
                let mut spawn_server = Command::new("java");
                spawn_server
                    .args([
                        xms_arg,
                        xmx_arg,
                        "-jar".to_string(),
                        jar_path.display().to_string(),
                        "nogui".to_string(),
                    ])
                    .current_dir(command_path)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .before_exec(|| {
                        libc::setsid();
                        Ok(())
                    });

                let process = spawn_server.spawn().expect("Failed to spawn Java process");

                thread::sleep(Duration::from_secs(5));

                let jps = Command::new("jps").arg("-l").output().expect("Failed to run jps");
                let jps_str = String::from_utf8_lossy(&jps.stdout).to_lowercase();

                if jps_str.contains(&command_path.to_string_lossy().to_lowercase()) {
                    return Some(ServerHandle { process });
                }
            }
        }
    } else {
        if cfg_app_data.system.os.to_lowercase().contains("windows") {
            #[cfg(windows)]
            {
                let process = Command::new(jar_path)
                    .arg("nogui")
                    .current_dir(command_path)
                    .creation_flags(0x08000000) // CREATE_NO_WINDOW
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("Failed to spawn Server via run.bat");

                thread::sleep(Duration::from_secs(20));

                let jps = Command::new("jps").arg("-l").output().expect("Failed to run jps");
                let jps_str = String::from_utf8_lossy(&jps.stdout).to_lowercase();

                if jps_str.contains("forge") || jps_str.contains("mod") {
                    return Some(ServerHandle { process });
                }
            }
        } else if cfg_app_data.system.os_mini.to_lowercase().contains("unix") {
            #[cfg(unix)]
            unsafe {
                let mut spawn_server = Command::new(jar_path);
                spawn_server
                    .arg("nogui")
                    .current_dir(command_path)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .before_exec(|| {
                        libc::setsid();
                        Ok(())
                    });

                let process = spawn_server.spawn().expect("Failed to spawn Server via run.sh");

                thread::sleep(Duration::from_secs(20));

                let jps = Command::new("jps").arg("-l").output().expect("Failed to run jps");
                let jps_str = String::from_utf8_lossy(&jps.stdout).to_lowercase();

                if jps_str.contains("forge") || jps_str.contains("mod") {
                    return Some(ServerHandle { process });
                }
            }
        }
    }

    None
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

        let cfg_app_str = read_cfg_silent();
        let mut cfg_app_data: Config = toml::from_str(&cfg_app_str)
        .expect("Could not parse TOML");

        let new_server_id = cfg_app_data.system.servers + 1;

        let base_dir = cfg_app_data.storage.directory.clone();

        let mut download_path = String::new();
        let mut toml_path = String::new();
        let mut dir_path = String::new();

        #[cfg(windows)] {
        download_path = format!("{}\\server{}\\server.jar", base_dir, new_server_id);
        toml_path = format!("{}\\server{}.toml", base_dir, new_server_id);
        dir_path = format!("{}\\server{}", base_dir, new_server_id);
        }
        #[cfg(unix)] {
        download_path = format!("{}/server{}/server.jar", base_dir, new_server_id);
        toml_path = format!("{}/server{}.toml", base_dir, new_server_id);
        dir_path = format!("{}/server{}", base_dir, new_server_id);
        }

        println!("What Minecraft Version would you like to download?");
        println!("A list of supported Versions can be found here:");
        println!("https://github.com/Delfi-CH/mc-server-manager-rust?tab=readme-ov-file#game-versions");
        println!("Type abort to abort.");
        let mut version = String::new();
        let mut modloader = String::new();
        let mut version_display = String::new();
        let mut modloader_display = String::new();
        let mut is_ml_set = false;
        loop {
        print!("-> ");

        io::stdout().flush().unwrap();

        io::stdin()
        .read_line(&mut version)
        .expect("Could not read the Input");
        version_display = version.trim().to_string();
        version = version.trim().to_lowercase();

        if version.contains("1.7.10") {
            break;
        } else if version.contains("abort") {
            println!("Aborting...");
            return;
        } else if version.contains("1.8.9") {
            break;
        } else if version.contains("1.9.4") {
            break;
        } else if version.contains("1.10.2") {
            break;
        } else if version.contains("1.11.2") {
            break;
        } else if version.contains("1.12.2") {
            break;
        } else if version.contains("1.13.2") {
            break;
        } else if version.contains("1.14.4") {
            break;
        } else if version.contains("1.15.2") {
            break;
        } else if version.contains("1.16.") {
            break;
        } else if version.contains("1.17.1") {
            break;
        } else if version.contains("1.18.2") {
            break;
        } else if version.contains("1.19.4") {
            break;
        } else if version.contains("1.20.") {
            break;
        } else if version.contains("1.21.") {
            break;
        } else if version.contains("craftmine") {
            modloader = "vanilla".to_string();
            modloader_display = "Vanilla".to_string();
            is_ml_set = true;
            break;
        } else if version.contains("potato") {
            modloader = "vanilla".to_string();
            modloader_display = "Vanilla".to_string();
            is_ml_set = true;
            break;
        } else if version.contains("a_or_b") {
            modloader = "vanilla".to_string();
            modloader_display = "Vanilla".to_string();
            is_ml_set = true;
            break;
        } else if version.contains("oneblockatatime") {
            modloader = "vanilla".to_string();
            modloader_display = "Vanilla".to_string();
            is_ml_set = true;
            break;
        } else if version.contains("infinite") {
            modloader = "vanilla".to_string();
            modloader_display = "Vanilla".to_string();
            is_ml_set = true;
            break;
        } else if version.contains("1.RV-Pre1") {
            modloader = "vanilla".to_string();
            modloader_display = "Vanilla".to_string();
            is_ml_set = true;
            break;
        } else if version.contains("15w14a") {
            modloader = "vanilla".to_string();
            modloader_display = "Vanilla".to_string();
            is_ml_set = true;
            break;
        } else {
            println!("Version {} is not supported! Please enter a valid Version!", version);
            version = "".to_string();
        }
    
        }


        println!("What Modloader would you like to use?");
        println!("Supported Modloaders are: Vanilla, Forge, Neoforge, Fabric, Paper and Folia.");
        println!("Note that not all Minecraft Versions are supported by every Modloader.");
        println!("For more info look here:");
        println!("https://github.com/Delfi-CH/mc-server-manager-rust?tab=readme-ov-file#modloaders");
        println!("Type abort to abort.");
        
        while is_ml_set == false {

        
        print!("-> ");

        io::stdout().flush().unwrap();

        io::stdin()
        .read_line(&mut modloader)
        .expect("Could not read the Input");
        modloader_display = modloader.trim().to_string();
        modloader = modloader.trim().to_lowercase();

        if modloader == "vanilla" {
            is_ml_set = true;
            break;
        } else if modloader == "forge" {
            is_ml_set = true;
            break;
        } else if modloader == "neoforge" {
            is_ml_set = true;
            break;
        }  else if modloader == "paper" {
            is_ml_set = true;
            break;
        }  else if modloader == "fabric" {
            is_ml_set = true;
            break;
        }  else if modloader == "abort" {
            is_ml_set = true;
            println!("Aborting...");
            return;
        }   else if modloader == "folia" {
            is_ml_set = true;
            break;
        }  else {
            println!("Modloader is not supported! Please enter a supported Modloader!");
            println!("Supported Modloaders are: Vanilla, Forge, Neoforge, Fabric and PaperMC.");
            modloader = "".to_string();
        }

        }

        let mut dotpath: PathBuf = home_dir().expect("Could not get home dir");

        dotpath.push(".mc-server-manager");

        let mut mcsvdl_path: PathBuf = home_dir().expect("Could not get home dir");

        mcsvdl_path.push(".mc-server-manager");

        #[cfg(windows)]
        mcsvdl_path.push("mcsvdl.exe");

        #[cfg(unix)]
        mcsvdl_path.push("mcsvdl");

        cfg_app_data.mcsvdl.mcsvdl_path = mcsvdl_path.display().to_string();

        write_cfg(&cfg_app_data, "config.toml");

        let mut mcsvdl_tar: PathBuf = home_dir().expect("Could not get home dir");

        mcsvdl_tar.push(".mc-server-manager");

        #[cfg(windows)]
        mcsvdl_tar.push("windows.zip");

        #[cfg(unix)]
        mcsvdl_tar.push("linux.tar");
        if fs::exists(download_path.clone()).expect("Could not check existance of Directory") == true {
            fs::remove_file(download_path.clone()).expect("Could not delete file");
        }
        if fs::exists(toml_path.clone()).expect("Could not check existance of Directory") == true {
            fs::remove_file(toml_path.clone()).expect("Could not delete file");
        } 
        if fs::exists(dir_path.clone()).expect("Could not check existance of Directory") == true {
            fs::remove_dir_all(dir_path.clone()).expect("Could not delete file");
            fs::create_dir(dir_path.clone()).expect("Could not create Directory.");
        } else {
            fs::create_dir(dir_path.clone()).expect("Could not create Directory.");
        }
        check_mcsvdl(dotpath, mcsvdl_tar);
        println!("Downloading Server.jar for {} {} to {} ...", modloader_display, version_display ,download_path);
        
        dl_server(mcsvdl_path, version.clone(), modloader.clone(), dir_path.clone());

        let mut eulapath = String::new();
        #[cfg(windows)] {
            eulapath = dir_path.clone() + "\\eula.txt"
        }
        #[cfg(unix)] {
            eulapath = dir_path.clone() + "/eula.txt";
        }

        println!("Creating eula.txt...");

        let mut eulafile = File::create(eulapath).expect("Could not create eula.txt");

        eulafile
            .write_all("eula = true".as_bytes())
            .expect("Could not write to file");

        let mut path_windows_dir = String::new();
        let mut path_windows_jar = String::new();
        let mut path_unix_dir = String::new();
        let mut path_unix_jar = String::new();

        let portnum = PORT_DEFAULT + cfg_app_data.system.servers;

        println!("Creating default server.properties...");

        let mut props_path = String::new();
        #[cfg(windows)] {
            props_path = win_path_cleaner(&dir_path).to_string() + "\\server.properties";
        }
        #[cfg(target_os = "linux")] {
            props_path = dir_path.to_string() + "/server.properties";
        }

        println!("{}", props_path);

        Command::new("curl")
            .args([
                "-L",
                "https://raw.githubusercontent.com/Delfi-CH/mc-server-manager-rust/refs/heads/main/data/server.properties",
                "-o",
                &props_path,
            ])
            .output()
            .expect("Failed to download File");

        let mut props_read = read_properties_hashmap(props_path.clone());

        if props_read.get("100").map_or(false, |v| v.contains("Error")) {
                println!("An Error occurred while reading server.properties!");
        } else {
            println!("Writing Port {} to server.properties", portnum);
            props_read.insert("query.port".to_string(), portnum.to_string());
            props_read.insert("server-port".to_string(), portnum.to_string());

            let props_file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&props_path)
                .expect("Failed to open file for writing");

            let mut writer = BufWriter::new(props_file);

            for (key, value) in &props_read {
                writeln!(writer, "{}={}", key, value).expect("Could not write to file.");
            }
            writer.flush().expect("Failed to flush the buffer");
            props_read = read_properties_hashmap(props_path.clone());
            println!("Wrote Port {} to server.properties", props_read.get("server-port").expect("Could not read Port"));
        }

        if modloader == "fabric" {
            #[cfg(windows)] {
                path_windows_dir = win_path_cleaner(&dir_path).to_string();
                path_windows_jar = win_path_cleaner(&dir_path).to_string() + "\\fabric-server-launch.jar";
                path_unix_dir = "File was downloaded on Windows. Please add the path manually".to_string();
                path_unix_jar = "File was downloaded on Windows. Please add the path manually".to_string();
            }
            #[cfg(unix)] {
            path_windows_dir = "File was downloaded on Unix or a Unix-like OS (probably Linux). Please add the path manually".to_string();
            path_windows_jar = "File was downloaded on Unix or a Unix-like OS (probably Linux). Please add the path manually".to_string();
            path_unix_dir = dir_path.clone();
            path_unix_jar = dir_path.clone() + "/fabric-server-launch.jar";
            }

        } else if modloader.contains("forge"){
            #[cfg(windows)] {
                path_windows_dir = win_path_cleaner(&dir_path).to_string();
                path_windows_jar = win_path_cleaner(&dir_path).to_string() + "\\run.bat";
                path_unix_dir = "File was downloaded on Windows. Please add the path manually".to_string();
                path_unix_jar = "File was downloaded on Windows. Please add the path manually".to_string();
            }
            #[cfg(unix)] {
            path_windows_dir = "File was downloaded on Unix or a Unix-like OS (probably Linux). Please add the path manually".to_string();
            path_windows_jar = "File was downloaded on Unix or a Unix-like OS (probably Linux). Please add the path manually".to_string();
            path_unix_dir = dir_path.clone();
            path_unix_jar = dir_path.clone() + "/run.sh";
            }

        } else {

        #[cfg(windows)] {
        path_windows_dir = win_path_cleaner(&dir_path).to_string();
        path_windows_jar = win_path_cleaner(&download_path).to_string();
        path_unix_dir = "File was downloaded on Windows. Please add the path manually".to_string();
        path_unix_jar = "File was downloaded on Windows. Please add the path manually".to_string();
        }
        #[cfg(unix)] {
        path_windows_dir = "File was downloaded on Unix or a Unix-like OS (probably Linux). Please add the path manually".to_string();
        path_windows_jar = "File was downloaded on Unix or a Unix-like OS (probably Linux). Please add the path manually".to_string();
        path_unix_dir = dir_path.clone();
        path_unix_jar = download_path.clone();
        }
        }
        println!("Creating .toml File for the server...");
        create_server_toml(toml_path.clone(), "server".to_string() + &new_server_id.to_string(), version, modloader, path_windows_dir, path_unix_dir, path_windows_jar, path_unix_jar, MIN_MEM_DEFAULT, MAX_MEM_DEFAULT, agree_eula ,portnum);
        println!("Adding .toml file to the configuration...");
        add_server_silent(toml_path.as_str());
        println!("Finished!");
    }
}
        
fn start_toml() {
    let cfg_app_str = read_cfg_silent();
    let cfg_app_data: Config = toml::from_str(&cfg_app_str).expect("Could not parse TOML");

    if cfg_app_data.system.servers == 0 {
        println!("No Server found!");
        println!("Please add a Server via the add action.");
        return;
    }

    let server_list_map: IndexMap<String, String> = cfg_app_data.server_list.server_list;

    let mut server_names = Vec::new();

    println!("Available Servers:");
    println!(); 
    let mut i = 1;

    for (key, path) in server_list_map.iter() {
        let cfg_server_str =
            fs::read_to_string(path).expect("Could not read server config file");
        let cfg_server_toml: ServerConfigFile =
            toml::from_str(&cfg_server_str).expect("Could not parse server TOML");

        let server_num = key.strip_prefix("server").expect("Could not remove server from key.");

        if !cfg_server_toml.server_config.running {
            server_names.push(server_num.to_string());
            println!("{}=>{}",i, key);
            i += 1;
        }
    }

    

    println!("\nWhat server do you want to start?");
    println!("Please enter a number.");
    println!("Or type 'abort' to exit.");

    let mut input = String::new();
    let selected_server_name: String;

    loop {
        print!("-> ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).expect("Could not read input");
        let trimmed = input.trim().to_lowercase();

        if trimmed == "abort" {
            println!("Exiting starting process...");
            return;
        }

        if let Ok(index) = trimmed.parse::<usize>() {
            if index >= 1 && index <= server_names.len() {
                selected_server_name = "server".to_owned() + &server_names[index-1];
                break;
            } else {
                println!("Invalid number, please select from the list.");
            }
        } else {
            println!("Please enter a valid number.");
        }
    }

    println!("Selected {}", selected_server_name);

    if let Some(server_toml_path) = server_list_map.get(&selected_server_name) {

        let cfg_server_str =
            fs::read_to_string(server_toml_path).expect("Could not read server config file");
        let mut cfg_server_toml: ServerConfigFile =
            toml::from_str(&cfg_server_str).expect("Could not parse server TOML");

        let (path_jar_str, path_dir_str) = if cfg_app_data.system.os_mini == "win" {
            (
                cfg_server_toml.server_config.path_windows_jar.clone(),
                cfg_server_toml.server_config.path_windows_dir.clone(),
            )
        } else {
            (
                cfg_server_toml.server_config.path_unix_jar.clone(),
                cfg_server_toml.server_config.path_unix_dir.clone(),
            )
        };

        let mut agree_eula = cfg_server_toml.server_config.eula;
        let mut eula_path = String::new();
        #[cfg(windows)] {
        eula_path = format!("{}\\eula.txt", path_dir_str);
        }
        #[cfg(unix)] {
        eula_path = format!("{}/eula.txt", path_dir_str);
        }
        let mut props_path = String::new();
        #[cfg(windows)] {
        props_path = format!("{}\\server.properties", path_dir_str);
        }
        #[cfg(unix)] {
        props_path = format!("{}/server.properties", path_dir_str);
        }

        let props_map = read_properties_hashmap(props_path);
        if let Ok(contents) = fs::read_to_string(&eula_path) {
            if contents.contains("eula = true") {
                agree_eula = true;
            }
        }

        if !agree_eula {
            println!("Do you agree to the Minecraft EULA?");
            println!("https://www.minecraft.net/en-us/eula");
            println!("y/n/open");

            loop {
                print!("-> ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_lowercase();

                match input.as_str() {
                    "y" => {
                        agree_eula = true;
                        break;
                    }
                    "n" => {
                        println!("You must agree to the EULA to start the server.");
                        return;
                    }
                    "open" => {
                        if let Err(e) =
                            open::that("https://www.minecraft.net/en-us/eula")
                        {
                            eprintln!("Failed to open browser: {}", e);
                        }
                    }
                    _ => println!("Not a valid input"),
                }
            }

            let _ = fs::write(&eula_path, "eula = true");
        }

        let mem_min: u32 = cfg_server_toml.server_config.min_mem.try_into().unwrap();
        let mem_max: u32 = cfg_server_toml.server_config.max_mem.try_into().unwrap();

        let path_to_jar = Path::new(&path_jar_str);
        let path_server_dir = Path::new(&path_dir_str);

        let is_fml = cfg_server_toml.server_config.modloader.contains("forge");

        println!("Starting Server...");

        let server_handle =
            start_generic(path_to_jar, path_server_dir, mem_min, mem_max, agree_eula, is_fml);

        if let Some(handle) = server_handle {
            let pid = handle.pid();
            println!("Server started successfully with PID {}!", pid);
            let mut registry = SERVER_REGISTRY.lock().unwrap();
            registry.insert(handle.pid().to_string(), handle);

            cfg_server_toml.server_config.running = true;
            cfg_server_toml.server_config.pid = pid.to_string(); 
            write_server_toml(&cfg_server_toml, &server_toml_path);

        } else {
            println!("An error occurred while starting the server!");
            println!(
                "Try running:\njava -Xmx{}M -Xms{}M -jar {}, in the directory: {}",
                mem_max,
                mem_min,
                path_to_jar.display(),
                path_server_dir.display(),
            );
        }
    } else {
        println!("Server not found in config.");
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
fn list_servers(){

    let jps = Command::new("jps").arg("-l").output().expect("Failed to list Java processes");
    let jps_str = String::from_utf8_lossy(&jps.stdout).to_lowercase();
    let cfg_app_str = read_cfg_silent();
    let cfg_app_data: Config = toml::from_str(&cfg_app_str)
        .expect("Could not parse TOML");

    let server_list_map: &IndexMap<String, String> = &cfg_app_data.server_list.server_list;

    let mut curr_server_count = 0;

    let max_server_count = cfg_app_data.system.servers;

    if curr_server_count == max_server_count {
        println!("No Servers found!");
        println!("Please add / download a Server.");
        return;
    } else if curr_server_count >= max_server_count {
        println!("You have a broken config file!");
        println!("Regenerate the config with newcfg.");
        return;
    }
    while curr_server_count != max_server_count {

        curr_server_count = curr_server_count + 1;

        let server_name = String::from("server".to_string() + &curr_server_count.to_string());

        let server_toml_path = server_list_map.get(&server_name).expect("Could not get Path to server.toml");

        let cfg_server_str =
            fs::read_to_string(server_toml_path).expect("Could not read server config file");
        let cfg_server_toml: ServerConfigFile =
            toml::from_str(&cfg_server_str).expect("Could not parse server TOML");

        let mut f_running = String::new();

        let mut is_running = false;

        if cfg_server_toml.server_config.pid == "" {
            is_running = false;
        } else if jps_str.contains(&cfg_server_toml.server_config.pid) {
            is_running = true;
        } else if jps_str.contains(".mods.") {
            is_running = true;
        } else if jps_str.contains("forge") {
            is_running = true;
        }

        if is_running == true {
            f_running = "Yes".to_string();
        } else {
            f_running = "No".to_string();
        }

        println!("{}", server_name);
        if is_running == true {
        println!("Name: {}, Version: {}, Modloader: {}, Currently running?: {}, with PID: {}, Has Port: {}", cfg_server_toml.server_config.name, cfg_server_toml.server_config.version, cfg_server_toml.server_config.modloader, f_running, cfg_server_toml.server_config.pid ,cfg_server_toml.server_config.port);
        } else {
        println!("Name: {}, Version: {}, Modloader: {}, Currently running?: {}, Has Port: {}", cfg_server_toml.server_config.name, cfg_server_toml.server_config.version, cfg_server_toml.server_config.modloader, f_running, cfg_server_toml.server_config.port);
        }

    }
}

fn create_server_toml(
    toml_path: String,     
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
    port: i32,)  {

    let mut server_toml = File::create(toml_path).expect("Could not create file");

    server_toml.write_all("title = \"server_config\"\n".as_bytes())
    .expect("Could not write to file");
    server_toml.write_all("\n".as_bytes())
    .expect("Could not write to file");
    server_toml.write_all("[server_config]\n".as_bytes())
    .expect("Could not write to file");
    server_toml.write_all(format!("name = \"{}\" \n", name.trim()).as_bytes())
    .expect("Could not write to file");
    server_toml.write_all(format!("version = \"{}\" \n", version.trim()).as_bytes())
    .expect("Could not write to file");
    server_toml.write_all(format!("modloader = \"{}\" \n", modloader.trim()).as_bytes())
    .expect("Could not write to file");
    server_toml.write_all(format!("path_windows_dir = \'{}\' \n", path_windows_dir.trim()).as_bytes())
    .expect("Could not write to file");
    server_toml.write_all(format!("path_windows_jar = \'{}\' \n", path_windows_jar.trim()).as_bytes())
    .expect("Could not write to file");
    server_toml.write_all(format!("path_unix_dir = \'{}\' \n", path_unix_dir.trim()).as_bytes())
    .expect("Could not write to file");
    server_toml.write_all(format!("path_unix_jar = \'{}\' \n", path_unix_jar.trim()).as_bytes())
    .expect("Could not write to file");
    server_toml.write_all(format!("min_mem = {} \n", min_mem.to_string().trim()).as_bytes())
    .expect("Could not write to file");
    server_toml.write_all(format!("max_mem = {} \n", max_mem.to_string().trim()).as_bytes())
    .expect("Could not write to file");
    server_toml.write_all(format!("eula = {} \n", eula.to_string().trim()).as_bytes())
    .expect("Could not write to file");
    server_toml.write_all("running = false\n".as_bytes())
    .expect("Could not write to file");
    server_toml.write_all("pid = \"\"\n".as_bytes())
    .expect("Could not write to file");
    server_toml.write_all(format!("port = {} \n", port.to_string().trim()).as_bytes())
    .expect("Could not write to file");

    
}

fn check_mcsvdl(dotpath: PathBuf, mcsvdl_tar: PathBuf) {

    let mut has_mcsvdl = false;

    println!("Checking for Helper Script...");

    #[cfg(windows)] {
    match fs::metadata(dotpath.join("mcsvdl.exe")) {
            Ok(_) => {
            has_mcsvdl = true;
            println!("Checking for Updates...");  
            }
            Err(_) => {
                println!("Helper Script wasnt found!...");  
            }
    }}
    #[cfg(unix)] {
    match fs::metadata(dotpath.join("mcsvdl")) {
            Ok(_) => {
            has_mcsvdl = true;
            println!("Checking for Updates...");  
            }
            Err(_) => {
                println!("Helper Script wasnt found!...");  
            }
    }}
        

    let cfg_app_str = read_cfg_silent();
    let mut cfg_app_data: Config = toml::from_str(&cfg_app_str)
    .expect("Could not parse TOML");

    let output = Command::new("curl")
        .arg("-s")
        .arg("https://api.github.com/repos/Delfi-CH/mc-server-downloader-py/releases/latest")
        .arg("-H")
        .arg("User-Agent: mc-sever-manager-rs")
        .output()
        .expect("Failed to execute curl");

    assert!(output.status.success(), "curl command failed");

    let stdout = String::from_utf8(output.stdout)
        .expect("Failed to convert curl output to UTF-8");

    let release_data: Release = serde_json::from_str(&stdout)
        .expect("Failed to parse JSON response");

    let release_num = release_data.tag_name;

    if has_mcsvdl != cfg_app_data.mcsvdl.has_mcsvdl {

        cfg_app_data.mcsvdl.has_mcsvdl = has_mcsvdl;
        write_cfg(&cfg_app_data, "config.toml");

    }
    

    if cfg_app_data.mcsvdl.has_mcsvdl == false {

        cfg_app_data.mcsvdl.has_mcsvdl = true;
        cfg_app_data.mcsvdl.mcsvdl_version = release_num;

        println!("Downloading Helper Script..."); 

        dl_mcsvdl(mcsvdl_tar, dotpath);

        write_cfg(&cfg_app_data, "config.toml");

        println!("Finished!"); 
            

    } else if release_num != cfg_app_data.mcsvdl.mcsvdl_version {

        println!("Updating Helper Script..."); 

        cfg_app_data.mcsvdl.mcsvdl_version = release_num;
        write_cfg(&cfg_app_data, "config.toml");

        dl_mcsvdl(mcsvdl_tar, dotpath);

    } else {
        println!("Helper Script up to date!");
    }
    
}

fn check_fml_vfile_updates(is_neoforge: bool) {

    let mut fml_vfile_path = String::new();
    let mut datadir = String::new();

    #[cfg(windows)] {
        fml_vfile_path = home_dir().expect("Could not get HomeDir").display().to_string();
        fml_vfile_path = fml_vfile_path + "\\.mc-server-manager\\data";
        datadir = home_dir().expect("Could not get HomeDir").display().to_string() + "\\.mc-server-manager\\data";
    }
    #[cfg(unix)] {
        fml_vfile_path = home_dir().expect("Could not get HomeDir").display().to_string();
        fml_vfile_path = fml_vfile_path + "/.mc-server-manager/data";
        datadir = home_dir().expect("Could not get HomeDir").display().to_string() + "/.mc-server-manager/data";
    }

    if is_neoforge == true {
        #[cfg(windows)] {
        fml_vfile_path = fml_vfile_path + "\\neofml_versions.toml"
        }
        #[cfg(unix)] {
        fml_vfile_path = fml_vfile_path + "/neofml_versions.toml"
        }
    } else {
        #[cfg(windows)] {
        fml_vfile_path = fml_vfile_path + "\\fml_versions.toml"
        }
        #[cfg(unix)] {
        fml_vfile_path = fml_vfile_path + "/fml_versions.toml"
        }
    }

    let mut fml_dl_path = String::new();
    if is_neoforge == true {
        fml_dl_path = "https://github.com/Delfi-CH/mc-server-manager-rust/blob/main/data/neofml_versions.toml".to_string();
    } else {
        fml_dl_path = "https://github.com/Delfi-CH/mc-server-manager-rust/blob/main/data/fml_versions.toml".to_string();
    }

    match fs::metadata(&fml_vfile_path) {
            Ok(_) => {
                // do nothing
            }
            Err(_) => {
                match fs::metadata(&datadir) {
                    Ok(_) => {
                        // do nothing
                    }
                    Err(_) => {
                        fs::create_dir(&datadir).expect("Could not create directory");
                    }
                }
            fml_vfile_donwload(is_neoforge, fml_vfile_path.clone());
        }
    }

    let fml_vfile_content = fs::read_to_string(&fml_vfile_path)
        .expect("Failed to read local file");

    let fml_vfile_gh = Command::new("curl")
        .arg("-s")
        .arg("-L")
        .arg(fml_dl_path)
        .output()
        .expect("Failed to execute curl");

    if !fml_vfile_gh.status.success() {
        eprintln!("Curl command failed with status: {:?}", fml_vfile_gh.status);
        std::process::exit(1);
    }

    let fml_vfile_gh_content = str::from_utf8(&fml_vfile_gh.stdout)
        .expect("Failed to parse curl output as UTF-8");

    if fml_vfile_content == fml_vfile_gh_content {
        println!("FML Info Files are up to date.");
    } else {
        println!("Updatimg FML Info Files...");
        fml_vfile_donwload(is_neoforge, fml_vfile_path.clone());
    }
}

fn fml_vfile_donwload(is_neoforge: bool, fml_file_path: String) {
    if is_neoforge == true {
        Command::new("curl")
            .args([
                "-L",
                "https://raw.githubusercontent.com/Delfi-CH/mc-server-manager-rust/refs/heads/main/data/neofml_versions.toml",
                "-o",
                &fml_file_path,
            ])
            .output()
            .expect("Failed to download File");
    } else {
        Command::new("curl")
            .args([
                "-L",
                "https://raw.githubusercontent.com/Delfi-CH/mc-server-manager-rust/refs/heads/main/data/fml_versions.toml",
                "-o",
                &fml_file_path,
            ])
            .output()
            .expect("Failed to download File"); 
    }
}

fn win_path_cleaner(path: &str) -> &str {
    if path.starts_with(r"\\?\") {
        &path[4..]
    } else {
        path
    }
}


fn win_path_cleaner_path(path: PathBuf) -> PathBuf {
    let s = path.to_string_lossy();
    if s.starts_with(r"\\?\") {
        PathBuf::from(&s[4..])
    } else {
        path.to_path_buf()
    }
}

fn dl_mcsvdl(mcsvdl_tar: PathBuf, dotpath: PathBuf) {
            #[cfg(windows)] {
            Command::new("curl")
                .args(&[
                "-L",    
                "https://github.com/Delfi-CH/mc-server-downloader-py/releases/latest/download/windows.zip",
                "-o",
                &mcsvdl_tar.display().to_string(),
                ])
                .current_dir(&dotpath)
                .output()
                .expect("Failed to download File");
            Command::new("tar") 
                .args(&[
                "-xf",
                &mcsvdl_tar.display().to_string(),
                ])
                .current_dir(&dotpath)
                .output()
                .expect("Failed to extract File");
            fs::remove_file(mcsvdl_tar).expect("Failed to remove Archive");
            fs::remove_file(dotpath.join("LICENSE")).expect("Failed to remove File");
                }
            #[cfg(unix)] {
            Command::new("curl")
                .args(&[
                "-L",    
                "https://github.com/Delfi-CH/mc-server-downloader-py/releases/latest/download/linux.tar",
                "-o",
                &mcsvdl_tar.display().to_string(),
                ])
                .current_dir(&dotpath)
                .output()
                .expect("Failed to download File");
            Command::new("tar") 
                .args(&[
                "-xf",
                &mcsvdl_tar.display().to_string(),
                ])
                .current_dir(&dotpath)
                .output()
                .expect("Failed to untar File");
            fs::remove_file(mcsvdl_tar).expect("Failed to remove Archive");
            fs::remove_file(dotpath.join("LICENSE")).expect("Failed to remove File");            
                }
}

fn stop_server(pid_raw: String) {

    let jps = Command::new("jps").arg("-l").output().expect("Failed to list Java processes");
    let jps_str_raw = String::from_utf8_lossy(&jps.stdout).trim().to_string();

    let jps_str: String = jps_str_raw
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let pid = pid_raw.trim();  

    if jps_str.contains(&pid) {
    #[cfg(windows)] {

    let output = Command::new("taskkill")
        .args(&["/PID", &pid, "/F"])
        .output()
        .expect("Failed to execute taskkill command");

    if output.status.success() {
        println!("Server stopped successfully.");
        return;
    } else {
        eprintln!("Failed to stop server.");
    }
    }

    #[cfg(unix)] {

    let output = Command::new("kill")
        .arg(&pid)
        .output()
        .expect("Failed to execute taskkill command");

    if output.status.success() {
        println!("Server stopped successfully.");
        return;
    } else {
        eprintln!("Failed to stop server.");
    }
    }
    
    } else {
        println!("PID {} doenst exist or is not a running Server!", pid_raw);
        println!("Consider running | jps -l | to get a list of active Java processes.");
    }
}

fn dl_server(mcsvdl_path: PathBuf, version: String, modloader: String, dir_path: String) {

    let mut fml_version= String::new();
        if modloader == "forge" {
            fml_version = fml_versions_str(version.clone(), false);

            println!("Downloading Forge installer...");
            Command::new(&mcsvdl_path)
            .args(&["-v", &version, "-m", &modloader, "-fv", &fml_version ])
            .current_dir(&dir_path)
            .output()
            .expect("Failed to download File");

            println!("Executing Installer...");
            Command::new("java")
            .args(&["-jar", "installer.jar", "--installServer"])
            .current_dir(&dir_path)
            .output()
            .expect("Could not apply the fabric installer!");

        } else if modloader == "neoforge" {
            fml_version = fml_versions_str(version.clone(), true);

            println!("Downloading NeoForge installer...");
            Command::new(&mcsvdl_path)
            .args(&["-v", &version, "-m", &modloader , "-nfv", &fml_version])
            .current_dir(&dir_path)
            .output()
            .expect("Failed to download File");

            println!("Executing Installer...");
            Command::new("java")
            .args(&["-jar", "installer.jar", "--install-server"])
            .current_dir(&dir_path)
            .output()
            .expect("Could not apply the fabric installer!");

        } else if modloader == "fabric" {

        println!("Downloading Fabric installer...");
        Command::new(&mcsvdl_path)
        .args(&["-v", &version, "-m", &modloader ])
        .current_dir(&dir_path)
        .output()
        .expect("Failed to download File");

        println!("Downloading Server.jar...");
        Command::new(&mcsvdl_path)
        .args(&["-v", &version, "-m", "vanilla" ])
        .current_dir(&dir_path)
        .output()
        .expect("Failed to download File");

        println!("Executing Installer...");
        Command::new("java")
        .args(&["-jar", "installer.jar", "server", &version])
        .current_dir(&dir_path)
        .output()
        .expect("Could not apply the fabric installer!");

        } else {
        Command::new(&mcsvdl_path)
        .args(&["-v", &version, "-m", &modloader ])
        .current_dir(&dir_path)
        .output()
        .expect("Failed to download File");
        }

}

fn stop_server_wrapper() {

    let (active_server_count, active_server_names_vec) = get_active_servers();
    let active_server_names = active_server_names_vec.join(" ");

    if active_server_count == 0 {
        println!("No Servers are currently running!");
        println!("Please Start a Server via the start action first.");
        return;
    } else if active_server_count <= 0 {
        println!("Something went wrong when getting all Servers. Please run the list action to get more information.");
        return;
    }

    println!("Active Servers: {}", active_server_count);
    println!();
    print_active_servers();
    println!();
    println!("What server do you want to stop?");
    println!("Please enter the Server number.");
    println!("Type abort to exit.");

    loop {
    print!("-> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read path");
    if input.contains("abort") {
        return;
    }
    input = "server".to_string() + &input.trim();

    if active_server_names.contains(&input) {
        let cfg_app_str = read_cfg_silent();
        let cfg_app_data: Config = toml::from_str(&cfg_app_str)
            .expect("Could not parse TOML");

        let server_list_map: &IndexMap<String, String> = &cfg_app_data.server_list.server_list;

        let server_toml_path = server_list_map.get(&input).expect("Could not get Path to the Server Config File");
        let cfg_server_str =
            fs::read_to_string(server_toml_path).expect("Could not read server config file");
        let mut cfg_server_toml: ServerConfigFile =
            toml::from_str(&cfg_server_str).expect("Could not parse server TOML");

        let pid = cfg_server_toml.server_config.pid;

        println!("Stopping {}...", input);

        stop_server(pid);

        cfg_server_toml.server_config.pid = "".to_string();
        cfg_server_toml.server_config.running = false;

        write_server_toml(&cfg_server_toml, &server_toml_path);

        return;

    } else {
        println!("No active Server found!");
    }
}
}

fn print_active_servers(){

    let jps = Command::new("jps").arg("-l").output().expect("Failed to list Java processes");
    let jps_str = String::from_utf8_lossy(&jps.stdout).to_lowercase();
    let cfg_app_str = read_cfg_silent();
    let cfg_app_data: Config = toml::from_str(&cfg_app_str)
        .expect("Could not parse TOML");

    let server_list_map: &IndexMap<String, String> = &cfg_app_data.server_list.server_list;

    let mut curr_server_count = 0;

    let max_server_count = cfg_app_data.system.servers;

    if curr_server_count == max_server_count {
        println!("No Servers found!");
        println!("Please add / download a Server.");
        return;
    } else if curr_server_count >= max_server_count {
        println!("You have a broken config file!");
        println!("Regenerate the config with newcfg.");
        return;
    }
    while curr_server_count != max_server_count {

        curr_server_count = curr_server_count + 1;

        let server_name = String::from("server".to_string() + &curr_server_count.to_string());

        let server_toml_path = server_list_map.get(&server_name).expect("Could not get Path to server.toml");

        let cfg_server_str =
            fs::read_to_string(server_toml_path).expect("Could not read server config file");
        let cfg_server_toml: ServerConfigFile =
            toml::from_str(&cfg_server_str).expect("Could not parse server TOML");

        let mut is_running = false;

        if cfg_server_toml.server_config.pid == "" {
            is_running = false;
        } else if jps_str.contains(&cfg_server_toml.server_config.pid) {
            is_running = true;
        }
        if is_running == true {
        println!("{}", server_name);
        println!("Name: {}, Version: {}, Modloader: {},  PID: {}, Has Port: {}", cfg_server_toml.server_config.name, cfg_server_toml.server_config.version, cfg_server_toml.server_config.modloader, cfg_server_toml.server_config.pid ,cfg_server_toml.server_config.port);
        }
        

    }
}

fn get_active_servers() -> (i32, Vec<String>) {

    let jps = Command::new("jps").arg("-l").output().expect("Failed to list Java processes");
    let jps_str = String::from_utf8_lossy(&jps.stdout).to_lowercase();
    let cfg_app_str = read_cfg_silent();
    let cfg_app_data: Config = toml::from_str(&cfg_app_str)
        .expect("Could not parse TOML");

    let server_list_map: &IndexMap<String, String> = &cfg_app_data.server_list.server_list;

    let mut curr_server_count = 0;

    let max_server_count = cfg_app_data.system.servers;

    let mut server_names = Vec::new();

    if curr_server_count == max_server_count {
        println!("No Servers found!");
        println!("Please add / download a Server.");
        return (-1, server_names);
    } else if curr_server_count >= max_server_count {
        println!("You have a broken config file!");
        println!("Regenerate the config with newcfg.");
        return (-1, server_names);
    }
    
    let mut has_printed = 0;
    while curr_server_count != max_server_count {

        curr_server_count = curr_server_count + 1;

        let server_name = String::from("server".to_string() + &curr_server_count.to_string());

        let server_toml_path = server_list_map.get(&server_name).expect("Could not get Path to server.toml");

        let cfg_server_str =
            fs::read_to_string(server_toml_path).expect("Could not read server config file");
        let mut cfg_server_toml: ServerConfigFile =
            toml::from_str(&cfg_server_str).expect("Could not parse server TOML");

        let mut is_running = false;

        if cfg_server_toml.server_config.pid == "" {
            is_running = false;
        } else if jps_str.contains(&cfg_server_toml.server_config.pid) {
            is_running = true;
            server_names.push(cfg_server_toml.server_config.name);
        } else if !jps_str.contains(&cfg_server_toml.server_config.pid) {
            is_running = false;
            cfg_server_toml.server_config.pid = "".to_string();
            cfg_server_toml.server_config.running = false;
            write_server_toml(&cfg_server_toml, &server_toml_path);
        }
        if is_running == true {
        has_printed = has_printed + 1;
        }

    }
    return (has_printed, server_names);
}

fn read_properties_hashmap(path: String) -> HashMap<String, String> {

    let prop_str = match fs::read(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read server config file: {}", e);
            let mut hash_err: HashMap<String, String> = HashMap::new();
            hash_err.insert("100".to_string(), "Error".to_string());
            return hash_err;
            
        }
    };
    
                
    let parsed = parse(&prop_str).unwrap();
    let properties = to_map(parsed);

    return properties;
}

fn read_properties() {
    let path = "C:\\Users\\laspi\\.mc-server-manager\\servers\\server1\\server.properties";

    let props = read_properties_hashmap(path.to_string());

    if props.get("100").map_or(false, |v| v.contains("Error")) {
        println!("An Error occurred while reading server.properties!");
    }

    let cfg_server_str =
            fs::read_to_string("C:\\Users\\laspi\\.mc-server-manager\\servers\\server1.toml").expect("Could not read server config file");
    let cfg_server_toml: ServerConfigFile =
            toml::from_str(&cfg_server_str).expect("Could not parse server TOML");

    println!("server-name={}",cfg_server_toml.server_config.name);
    println!("mc-version={}",cfg_server_toml.server_config.version);
    println!("modloader={}",cfg_server_toml.server_config.modloader);
    println!("is-running={}", cfg_server_toml.server_config.running);

    println!("{}", path);

    println!("allow-flight={}", props.get("allow-flight").expect("E"));
    println!("allow-nether={}", props.get("allow-nether").expect("E"));
    println!("difficulty={}", props.get("difficulty").expect("E"));
    println!("enable-command-block={}", props.get("enable-command-block").expect("E"));
    println!("white-list={}", props.get("white-list").expect("E"));
    println!("enforce-whitelist={}", props.get("enforce-whitelist").expect("E"));
    println!("force-gamemode={}", props.get("force-gamemode").expect("E"));
    println!("gamemode={}", props.get("gamemode").expect("E"));
    println!("generate-structures={}", props.get("generate-structures").expect("E"));
    println!("hardcore={}", props.get("hardcore").expect("E"));
    println!("level-seed={}", props.get("level-seed").expect("E"));
    println!("max-players={}", props.get("max-players").expect("E"));
    println!("motd={}", props.get("motd").expect("E"));
    println!("pvp={}", props.get("pvp").expect("E"));
    println!("view-distance={}", props.get("view-distance").expect("E"));
    println!("simulation-distance={}", props.get("simulation-distance").expect("E"));
    println!("spawn-monsters={}", props.get("spawn-monsters").expect("E"));
    println!("spawn-protection={}", props.get("spawn-protection").expect("E"));
}

fn remove_server(server_toml_path: String) -> (i32, bool) {

    // needs implenting
    let cfg_server_str =
            fs::read_to_string(server_toml_path).expect("Could not read server config file");
    let cfg_server_toml: ServerConfigFile =
            toml::from_str(&cfg_server_str).expect("Could not parse server TOML");
    
    if cfg_server_toml.server_config.running == true {
        return(2, false);
    } else if  cfg_server_toml.server_config.running == false{
        let cfg_app_str = read_cfg_silent();
        let mut cfg_app_data: Config = toml::from_str(&cfg_app_str).expect("Could not parse TOML");

        fs::remove_file(cfg_app_data.server_list.server_list.get(&cfg_server_toml.server_config.name).expect("Could not get Path")).expect("Could not remove Server Directory");
        
        cfg_app_data.server_list.server_list.swap_remove(&cfg_server_toml.server_config.name).expect("Could not delete Entry");

        #[cfg(windows)]{
        fs::remove_dir_all(cfg_server_toml.server_config.path_windows_dir).expect("Could not remove Server Directory");
        }
        #[cfg(unix)]{
        fs::remove_dir_all(cfg_server_toml.server_config.path_unix_dir).expect("Could not remove Server Directory");
        }
        return(0, true);

    }
    return (1, false);
}
fn list_servers_hash() -> HashMap<u32, String> {
    let jps = Command::new("jps").arg("-l").output().expect("Failed to list Java processes");
    let jps_str = String::from_utf8_lossy(&jps.stdout).to_lowercase();
    let mut jps_map: HashMap<u32, String> = HashMap::new();

    for line in jps_str.lines() {
        if let Some((pid_str, rest)) = line.split_once(' ') {
            if let Ok(pid) = pid_str.parse::<u32>() {
                jps_map.insert(pid, rest.to_string());
            }
        }
    }
    return jps_map;
}

fn read_server_stdout(server_id: &str) -> bool {
    let mut registry = SERVER_REGISTRY.lock().unwrap();

    if let Some(server_handle) = registry.get_mut(server_id) {
        // Take ownership of the stdout stream
        if let Some(stdout) = server_handle.process.stdout.take() {
            let id = server_id.to_string();
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line_result in reader.lines() {
                    match line_result {
                        Ok(line) => {
                            println!("[{} stdout] {}", id, line);
                        }
                        Err(e) => {
                            eprintln!("[{} stdout] Error reading stdout: {}", id, e);
                            break;
                        }
                    }
                }
            });
            true
        } else {
            eprintln!("Server '{}' has no stdout to read", server_id);
            false
        }
    } else {
        eprintln!("No server found with ID '{}'", server_id);
        false
    }
}