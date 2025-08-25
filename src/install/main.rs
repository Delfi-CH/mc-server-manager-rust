// SPDX-License-Identifier: GPL-3.0-or-later
use std::fs;
use std::process::Command;
use std::io::{self, Write};
use app_lib::*;
use dir::home_dir;
fn main() {
    println!();
    println!("APPNAME Installer V0.7");
    // TODO: think of a name and put it here
    println!("Press ENTER to install the [DEFAULT] option.");
    println!();
    println!("What Packages do you want to install?");
    println!("1 | Webapp-Backend [DEFAULT]");
    println!("2 | Command-Line App");
    println!("3 | All");

    let mut input1 = 0;
    
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
        input1 = 1;
        break;
    } else if input == "2" {
        println!("Selecting Command-Line App...");
        input1 = 2;
        break;
    } else if input == "3" {
        println!("Selecting both options...");
        input1 = 3;
        break;
    } else if input == "" {
        println!("Selecting Webapp-Backend...");
        input1 = 1;
        break;
    } else {
        println!("Not a valid input. Please try again.");
    }  
}
let mut port: u32=29900;
if input1 == 1 || input1 == 3 {
println!();
println!("Select the WebApp Port");
println!("[DEFAULT]: 29900");

loop {
    print!("->| ");
    io::stdout().flush().unwrap();

    let mut port_input = String::new();
    io::stdin()
        .read_line(&mut port_input)
        .expect("Could not read the input");

    let port_input = port_input.trim();

    if port_input == "" {
        println!("Selecting Port {}...", port);
        break;
    }

    match port_input.parse::<u32>() {
        Ok(port_input_u32) => {
            if port_input_u32 <= 1024 {
                println!("Cannot bind to Ports 1-1024. Please try again.");
            } else if port_input_u32 == 29001 {
                println!("Port 29001 is already in use for backend Services. Please try again.");
            } else if port_input_u32 > 65535 {
                println!("Port can't be bigger than 65535. Please try again.");
            } else {
                println!("Selecting Port {}...", port_input_u32);
                port = port_input_u32;
                break;
            }
            }
        Err(_) => {
            println!("Not a valid input. Please enter a numeric port.");
        }
        }
    }
}
println!("Summary: ");
println!("---------------");
println!("Packages: ");
if input1 == 1 {
    println!("- Webapp-Backend");
} else if input1 == 2 {
    println!("- Command-Line App");
} else if input1 == 3 {
    println!("- Webapp-Backend");
    println!("- Command-Line App");     
} else {
    println!("An Error occured.");
    println!("Please restart the Installer.");
    return;
}
println!("---------------");
println!("Configuration:");
if input1 == 1 || input1 == 3 {
println!("- Webapp Port: {}", port);
}
println!();
loop {
println!("Continiue? [Y/N]");
let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read the Input");

        let input= input.to_lowercase();
        let input = input.trim();

if input == "n" {
    println!("Exiting Installer...");
    return;
} else if input == "y" {
    break;
} else {
    println!("{} is not a valid Input", input);
}
}
println!("Starting installation...");
let has_curl = check_curl();

if has_curl == false {
    #[cfg(unix)] {
    println!("CURL is not installed!");
    println!("Install CURL via");
    
        if let Some(cmd) = install_curl_command() {
        println!("{}", cmd);
        }
    }
    #[cfg(windows)] {
        println!("CURL was not found!");
        println!("Your Windows Install seemes to corrupted!");
        println!("Please try to update / restore Windows.")
    }
    return;
} else {
    println!("CURL was found...");
    
}

println!("Creating directoriies...");
let mcsvman_dir = home_dir().expect("Could not get Home dir").join(".mc-server-manager");
if fs::metadata(&mcsvman_dir).is_ok() {
    if let Err(e) = fs::remove_dir_all(&mcsvman_dir) {
        eprintln!("Failed to remove directory: {}", e);
        return;
    }
}
if let Err(e) = fs::create_dir(&mcsvman_dir) {
    eprintln!("Failed to create directory: {}", e);
    return;
}
if let Err(e) = fs::create_dir(&mcsvman_dir.join("data")) {
    eprintln!("Failed to create directory: {}", e);
    return;
}
if let Err(e) = fs::create_dir(&mcsvman_dir.join("servers")) {
    eprintln!("Failed to create directory: {}", e);
    return;
}
if let Err(e) = fs::create_dir(&mcsvman_dir.join("bin")) {
    eprintln!("Failed to create directory: {}", e);
    return;
}

if input1 == 1 || input1 == 3 {
    println!("Downloading Webapp-Backend...");
    // Download when finished...
} 
if input1 == 2 || input1 == 3 {
    println!("Downloading Command-Line App...");
    // Download when finished...
}
println!("Downloading additional Components...");
dl_mcsvdl();
Command::new("curl")
    .args([
        "-L",
        "https://raw.githubusercontent.com/Delfi-CH/mc-server-manager-rust/refs/heads/main/data/neofml_versions.toml",
        "-o",
        &mcsvman_dir.join("data").join("neofml_versions.toml").display().to_string(),
        ])
    .output()
    .expect("Failed to download File");
Command::new("curl")
    .args([
        "-L",
        "https://raw.githubusercontent.com/Delfi-CH/mc-server-manager-rust/refs/heads/main/data/fml_versions.toml",
        "-o",
        &mcsvman_dir.join("data").join("fml_versions.toml").display().to_string(),
        ])
    .output()
    .expect("Failed to download File");

    println!("Creating configuration file...");
    create_config();
    println!("Done!");
}

fn check_curl() -> bool {
    Command::new("curl")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(unix)]
fn install_curl_command() -> Option<&'static str> {
    use std::fs;

    let os_release = fs::read_to_string("/etc/os-release").ok()?;
    if os_release.contains("ubuntu") || os_release.contains("debian") {
        Some("sudo apt-get update && sudo apt-get install -y curl")
    } else if os_release.contains("fedora") {
        Some("sudo dnf install -y curl")
    } else if os_release.contains("rhel") {
        Some("sudo yum install -y curl")
    } else if os_release.contains("arch") {
        Some("sudo pacman -Sy curl")
    } else if os_release.contains("alpine") {
        Some("sudo apk add curl")
    } else if os_release.contains("suse") {
        Some("sudo zypper install curl")
    } else {
        Some("Check your Package-Manager or Install from Source")
    }
}

