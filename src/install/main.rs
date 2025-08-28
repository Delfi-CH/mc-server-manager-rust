// SPDX-License-Identifier: GPL-3.0-or-later
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::io::{self, Write};
use app_lib::*;
use dir::home_dir;
use git2::Repository;
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
println!("Select a Port for the WebApp");
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

println!("How do you want to obatin these packages?");
println!("1 | Download [DEFAULT] (reccomended for most users.)");
println!("2 | Compile locally (reccomended only for expert users, requires rustc/cargo, python3 and git)");

let mut input2 = 0;

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
        println!("Selecting download...");
        input2 = 1;
        break;
    } else if input == "2" {
        println!("Selecting compilation..");
        input2 = 2;
        break;
    } else if input == "" {
        println!("Selecting download...");
        input2 = 1;
        break;
    } else {
        println!("Not a valid input. Please try again.");
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
if input2 == 1 {
    println!("- Downloading of components");
} else if input2 == 2{
    println!("- Local Compilation");
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

println!("Creating directories...");
let mcsvman_dir = home_dir().map(|path| path.join(".mc-server-manager")).unwrap_or_else(|| {
    eprintln!("Error: Could not determine the home directory.");
    std::process::exit(1);
});
if fs::metadata(&mcsvman_dir).is_ok() {
    if let Err(e) = fs::remove_dir_all(&mcsvman_dir) {
        eprintln!("Failed to remove directory: {}", e);
        std::process::exit(1);
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

if input2 == 1 {
    download(input1, mcsvman_dir);
} else {
    compile(input1);
}

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

fn check_cc() -> bool {

    let py: bool = check_py();
    let py3: bool = check_py3();;
    let cargo: bool = check_cargo();
    let git: bool = check_git();

    if ((py || py3) && cargo && git) == true {
        return true;
    } else {
        return false;
    }

}

fn check_py() -> bool {
    let output = Command::new("python")
        .arg("--version")
        .output();

    match output {
        Ok(output) => {
            let version_info = String::from_utf8_lossy(&output.stdout).to_string()
                + &String::from_utf8_lossy(&output.stderr);
            if version_info.contains("Python 3.") {
                true
            } else {
                eprintln!("Python found, but not version 3.x: {}", version_info);
                false
            }
        }
        Err(e) => {
            eprintln!("Failed to execute Python: {}", e);
            false
        }
    }
}


fn check_py3() -> bool {
    Command::new("python3")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
fn check_cargo() -> bool {
    Command::new("cargo")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
fn check_git() -> bool {
    Command::new("git")
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

fn download(input1: i32, mcsvman_dir: PathBuf) {
    println!("Download");
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
}

fn compile(input1: i32) {

    #[cfg(target_os = "linux")] {
        println!();
        println!("WARNING!");
        println!("Compiling REQUIRES:");
        println!("- OpenSSL Development Libraries");
        println!("- pkg-config");
        println!();
        println!("PLEASE CHECK FOR THESE BEFORE CONTINUING!");
        println!("ADDITIONAL INFO CAN BE FOUND HERE: https://github.com/Delfi-CH/mc-server-manager-rust/blob/main/COMPILING.md");
        loop {
            println!("CONTINIUE? [Y/N]");
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
    }

    println!("Checking for compilers & tools");

    if check_cc() == true {

    } else if (check_py() || check_py3())==false {
        println!("Python 3 is not installed!");
        println!("Please install Python 3 from https://www.python.org/downloads/ and retry.");
        std::process::exit(2);
    } else if !(check_cargo()) {
        println!("Rust is not installed!");
        println!("Please install Rust from https://www.rust-lang.org/tools/install and retry.");
        std::process::exit(2);
    } else if check_git()==false {
        println!("Git is not installed!");
        println!("Please install Git from https://git-scm.com/downloads and retry.");
        std::process::exit(2);
    }

    println!("All needed tools were found...");
    println!("Prepearing Build...");
    let src_dir = home_dir().map(|path| path.join(".mc-server-manager").join("src")).unwrap_or_else(|| {
    eprintln!("Error: Could not determine the home directory.");
    std::process::exit(1);  
    });
    if fs::metadata(&src_dir).is_ok() {
    if let Err(e) = fs::remove_dir_all(&src_dir) {
        eprintln!("Failed to remove directory: {}", e);
        std::process::exit(1);
    }
    }
    if let Err(e) = fs::create_dir(&src_dir) {
        eprintln!("Failed to create directory: {}", e);
    std::process::exit(1);
    }
    println!("Cloning Repositories...");
    let repo_rs_url = "https://github.com/Delfi-CH/mc-server-manager-rust";
    let repo_py_url = "https://github.com/Delfi-CH/mc-server-downloader-py";
    match Repository::clone(repo_rs_url, src_dir.join("mc-server-manager-rust")) {
        Ok(repo_rs) => repo_rs,
        Err(e) => panic!("Failed to clone the Repository https://github.com/Delfi-CH/mc-server-manager-rust: {}", e),
    };
    match Repository::clone(repo_py_url, src_dir.join("mc-server-downloader-py")) {
        Ok(repo_py) => repo_py,
        Err(e) => panic!("Failed to clone the Repository https://github.com/Delfi-CH/mc-server-downloader-py: {}", e),
    };
    println!("Compiling Rust binaries...");
    if input1 == 1 || input1 == 3 {
    println!("Compiling Webapp-Backend...");
        match compile_rust("webapp-backend".to_string(), src_dir.join("mc-server-manager-rust")) {
        Ok(_) => println!("Compiling of Webapp-Backend finished successfully..."),
        Err(e) => eprintln!("Error while compiling Webapp-Backend: {}", e),
    }
    } 
    if input1 == 2 || input1 == 3 {
        println!("Compiling Command-Line App...");
        match compile_rust("cli".to_string(), src_dir.join("mc-server-manager-rust")) {
        Ok(_) => println!("Compiling of Command-Line App finished successfully..."),
        Err(e) => eprintln!("Error while compiling Command-Line App: {}", e),
    
    }}
    println!("Compiling daemon...");
    match compile_rust("daemon".to_string(), src_dir.join("mc-server-manager-rust")) {
        Ok(_) => println!("Compiling of Daemon finished successfully..."),
        Err(e) => eprintln!("Error while compiling Daemon: {}", e),
    }
    println!("Preparing mcsvdl...");

    let pip_install= Command::new("pip")
    .args(&["install", "-r", "requirements.txt"])
    .current_dir(src_dir.join("mc-server-downloader-py"))
    .output();

    match pip_install {
        Ok(pip_install) => {
        if !pip_install.status.success() {
            eprintln!(
                "pip install failed with status: {}\nstdout: {}\nstderr: {}",
                pip_install.status,
                String::from_utf8_lossy(&pip_install.stdout),
                String::from_utf8_lossy(&pip_install.stderr)
            );
        } else {
            println!("Requirements installed successfully...");
        }
    }
    Err(e) => {
        eprintln!("Error while installing requirements: {}", e);
    }
    }
    println!("Compiling mcsvdl...");

    let mut mcsvdl_bin_name = "mcsvdl";
    #[cfg(windows)] {
    mcsvdl_bin_name = "mcsvdl.exe";
    }

    match compile_mcsvdl(src_dir.join("mc-server-downloader-py"), mcsvdl_bin_name.to_string(), src_dir) {
        Ok(_) => println!("Compiling of mcsvdl finished successfully..."),
        Err(e) => eprintln!("Error while compiling mcsvdl: {}", e),
    }
}

fn compile_rust(bintype: String, path: PathBuf) -> Result<(), Box<dyn std::error::Error>>  {
    let output = Command::new("cargo")
        .args(&["build","--bin", &bintype ,  "--release"])
        .current_dir(path)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Err(format!("Build failed:\n{}", err_msg).into())
    }
}


// TODO: Fix Pyinstall on Linux
fn compile_mcsvdl(
    path: PathBuf,
    mcsvdl_bin_name: String,
    src_dir: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let python_cmd = if check_py() {
        "python"
    } else if check_py3() {
        "python3"
    } else {
        eprintln!("Python 3 is not installed!\nPlease install Python 3 from https://www.python.org/downloads/ and retry.");
        std::process::exit(2);
    };

    // Keep trying to install pyinstaller until successful
    let pyinstall_path = loop {
        let (installed, path) = check_pyinstall();
        if installed {
            break path;
        }

        match install_pyinstall(path.clone(), python_cmd.to_string()) {
            Ok(_) => println!("Installation of pyinstaller completed successfully..."),
            Err(e) => {
                eprintln!("Error while installing pyinstaller: {}", e);
                std::process::exit(2);
            }
        }
    };

    // Run pyinstaller build command
    let pyinstall_build = Command::new(&pyinstall_path)
        .args(&["--clean", "--onefile", "main.py", "--name", &mcsvdl_bin_name])
        .current_dir(src_dir.join("mc-server-downloader-py"))
        .output()?;

    if !pyinstall_build.status.success() {
        eprintln!(
            "pyinstaller failed with status: {}\nstdout: {}\nstderr: {}",
            pyinstall_build.status,
            String::from_utf8_lossy(&pyinstall_build.stdout),
            String::from_utf8_lossy(&pyinstall_build.stderr)
        );
        return Err("PyInstaller build failed".into());
    }

    Ok(())
}


fn install_pyinstall(path: String, python_cmd: String) -> Result<(), Box<dyn std::error::Error>> {
    let install_pyinstall = Command::new(python_cmd)
        .args(&["-m", "pip", "install", "--user", "pyinstaller"])
        .current_dir(path)
        .output()?;

    if install_pyinstall.status.success() {
        Ok(())
    } else {
        let err_msg = String::from_utf8_lossy(&install_pyinstall.stderr);
        Err(format!("Could not install Pyinstaller:\n{}", err_msg).into())
    }
}

fn get_newest_python_dir_win(path: PathBuf) -> io::Result<Option<String>> {
    let mut max_version: Option<u32> = None;
    let mut max_dir_name: Option<String> = None;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("Python3") {
                    let suffix = &name["Python3".len()..];
                    if let Ok(num) = suffix.parse::<u32>() {
                        if max_version.map_or(true, |current_max| num > current_max) {
                            max_version = Some(num);
                            max_dir_name = Some(name.to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(max_dir_name)
}

fn check_pyinstall() -> (bool, String) {
    let mut pyinstall_path: PathBuf = Default::default();
    #[cfg(unix)] {
    pyinstall_path = home_dir()
        .unwrap_or_else(|| PathBuf::from("/"))
        .join(".local/bin/pyinstaller");
}

    #[cfg(windows)] {
        let appdata = match env::var("AppData") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: Could not read %AppData% environment variable: {}", e);
            std::process::exit(2);
            }
        };

        pyinstall_path = PathBuf::from(appdata);
        pyinstall_path.push("Python");
        let pydir_win = match get_newest_python_dir_win(pyinstall_path.clone()) {
            Ok(Some(dir)) => Some(dir),
            Ok(None) => {
                eprintln!("No Python directory found at {}", pyinstall_path.clone().display());
                None
            }
            Err(e) => {
                eprintln!("Could not check Dir {}: {}", pyinstall_path.clone().display(), e);
                None
            }
        };
        match pydir_win {
            None => {
                eprintln!("Dir is empty!");
                std::process::exit(2);
            },
            Some(dir) => {
                pyinstall_path.push(dir);
        },
        }

        pyinstall_path.push("Scripts");
        pyinstall_path.push("pyinstaller.exe");
    }
    
    println!("{}", pyinstall_path.display());
    let pyinstall_check = match Command::new(pyinstall_path.clone()).arg("-v").output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to execute Pyinstaller: {}", e);
            return (false, pyinstall_path.clone().display().to_string());
        }
    };

    if pyinstall_check.status.success() {
        (true, pyinstall_path.clone().display().to_string())
    } else {
        let err_msg = String::from_utf8_lossy(&pyinstall_check.stderr);
        eprintln!("Could not run Pyinstaller:\n{}", err_msg);
        (false, pyinstall_path.clone().display().to_string())
    }
}