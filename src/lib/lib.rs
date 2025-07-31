// Imports
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use std::collections::HashMap;
use dir::home_dir;
#[cfg(windows)]
use winver::*;

// Sanity Check

pub fn sanity_check() -> String {
    return "This works".to_string();
}

fn fallback_path() -> PathBuf {
    //Gets called when HomeDir doesnt exist for some Reason.
    // TODO: Replace MyAPP with a good name
    #[cfg(windows)]
    {
        PathBuf::from(r"C:\Users\Public\myapp")
    }

    #[cfg(unix)]
    {
        PathBuf::from("/var/tmp/myapp")
    }
}

pub fn get_dotpath() -> PathBuf {

    let mut dotpath = home_dir().unwrap_or(fallback_path());
    dotpath = dotpath.join(".mc-server-manager");
    return dotpath;
}

// MCSVDL stuff

#[cfg(unix)]
fn check_mcsvdl_unix_elf() -> bool {
    Command::new(get_dotpath().join("bin").join("mcsvdl"))
        .arg("--help")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
#[cfg(windows)]
fn check_mcsvdl_win_pe() -> bool {
    Command::new(get_dotpath().join("bin").join("mcsvdl.exe"))
        .arg("--help")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn dl_mcsvdl() {
    #[cfg(unix)]
let mcsvdl_tar_path = get_dotpath().join("bin").join("linux.tar");
#[cfg(windows)]
let mcsvdl_tar_path = get_dotpath().join("bin").join("windows.zip");

#[cfg(windows)] {
    Command::new("curl")
        .args(&[
            "-L",    
            "https://github.com/Delfi-CH/mc-server-downloader-py/releases/latest/download/windows.zip",
            "-o",
            &mcsvdl_tar_path.display().to_string(),
            ])
        .current_dir(get_dotpath().join("bin"))
        .output()
        .expect("Failed to download File");
    Command::new("tar") 
        .args(&[
            "-xf",
            &mcsvdl_tar_path.display().to_string(),
            ])
        .current_dir(get_dotpath().join("bin"))
        .output()
        .expect("Failed to extract File");
    fs::remove_file(&mcsvdl_tar_path).expect("Failed to remove Archive");
    fs::remove_file(get_dotpath().join("bin").join("LICENSE")).expect("Failed to remove File");
}
#[cfg(unix)] {
    Command::new("curl")
        .args(&[
            "-L",    
            "https://github.com/Delfi-CH/mc-server-downloader-py/releases/latest/download/linux.tar",
            "-o",
            &mcsvdl_tar_path.display().to_string(),
            ])
        .current_dir(get_dotpath().join("bin"))
        .output()
        .expect("Failed to download File");
    Command::new("tar") 
        .args(&[
            "-xf",
            &mcsvdl_tar_path.display().to_string(),
            ])
        .current_dir(get_dotpath().join("bin"))
        .output()
        .expect("Failed to extract File");
    fs::remove_file(&mcsvdl_tar_path).expect("Failed to remove Archive");
    fs::remove_file(get_dotpath().join("bin").join("LICENSE")).expect("Failed to remove File");
}
}

// config.toml stuff

// checking stuff

pub fn get_config_path() -> PathBuf {

    let mut cfg_path = home_dir().unwrap_or(fallback_path());
    cfg_path = cfg_path.join(".mc-server-manager");
    cfg_path = cfg_path.join("config.toml");
    return cfg_path;
}
pub fn check_config_existance() -> bool {
    get_config_path().exists()
}

// Parsing OS Data

fn get_os() -> String {
    #[cfg(unix)]
    return "Unix/Unix-Like".to_string();
    #[cfg(windows)]
    return "Windows NT".to_string();
}
#[cfg(target_os = "linux")]
fn read_os_release() -> HashMap<String, String> {
    let mut info = HashMap::new();

    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                let value = value.trim_matches('"');
                info.insert(key.to_string(), value.to_string());
            }
        }
    } else {
        eprintln!("Warning: Could not read /etc/os-release");
    }

    info
}

fn get_os_details() -> String {
    let os_type = get_os();
    let mut os_ver = String::new();

    #[cfg(unix)] {
    let mut unix_subtype = String::new();
    if cfg!(target_os = "linux") {

        unix_subtype = ", Linux".to_string();

        let mut linux_dist = String::new();
        let os_release_file = read_os_release();
        let dist_name = os_release_file
            .get("NAME")
            .map_or("Unknown Distribution", |v| v.as_str());

        let os_ver = os_release_file
            .get("VERSION")
            .map_or("Unknown Version", |v| v.as_str());

        let linux_dist = format!("{} {}", dist_name, os_ver);
        return os_type + &unix_subtype + ", " + &linux_dist;

    } else if cfg!(target_os = "macos") {
        unix_subtype = ", MacOS".to_string();

        let mut macos_build = String::new();
        os_ver = "99.99".to_string(); // TODO: RELPACE WITH ACTUAL SOLUTION
        macos_build = "99999".to_string(); // TODO: RELPACE WITH ACTUAL SOLUTION
        return os_type + &unix_subtype + &os_ver + ", Build " + &macos_build;
    } else if cfg!(target_os = "freebsd") {
        unix_subtype = ", BSD (FreeBSD)".to_string();
        os_ver = "Unknown Version".to_string(); // TODO: RELPACE WITH ACTUAL SOLUTION
        return os_type + &unix_subtype + &os_ver;
    } else if cfg!(target_os = "openbsd") {
        unix_subtype = ", BSD (OpenBSD)".to_string();
        os_ver = "Unknown Version".to_string(); // TODO: RELPACE WITH ACTUAL SOLUTION
        return os_type + &unix_subtype + &os_ver;
    } else {
        unix_subtype = ", Unknown Unix Variant".to_string();
        os_ver = "Unknown Version".to_string();
        return os_type + &unix_subtype + &os_ver;
    }
    }

    #[cfg(windows)] {
    let mut win_build = String::new();
    
    let win_ver = WindowsVersion::detect().unwrap();
    if win_ver >= WindowsVersion::new(10, 0, 22000) {
        os_ver = " 11".to_string();
        win_build = win_ver.build.to_string();
    } else {
        os_ver = " 10".to_string();
        win_build = win_ver.build.to_string();
    }
    return os_type + &os_ver + ", Build " + &win_build;
}
}


pub fn create_config() {
    let mut mcsvdl_path = PathBuf::new();
    #[cfg(windows)]
    if check_mcsvdl_win_pe() == true {
        mcsvdl_path = get_dotpath().join("bin").join("mcsvdl.exe");
    }
    #[cfg(unix)]
    if check_mcsvdl_unix_elf() == true {
        mcsvdl_path = get_dotpath().join("bin").join("mcsvdl");
    }
    if check_config_existance() == true {
        fs::remove_file(get_config_path()).expect("Could not delete file");
    }
    let mut cfg_file = File::create(get_config_path()).expect("Could not create file");
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
                .write_all(format!("os_type = \"{}\" \n", get_os().trim()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("os_details = \"{}\" \n", get_os_details().trim()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("servers = 0\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("after_initial_setup = false\n".as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("data_path = \"{}\" \n", get_dotpath().join("data").display()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all("[mcsvdl]\n".as_bytes())
                .expect("Could not write to file");
            #[cfg(unix)]
            cfg_file
                .write_all(format!("has_mcsvdl = {}\n",check_mcsvdl_unix_elf()).as_bytes())
                .expect("Could not write to file");
            #[cfg(windows)]
            cfg_file
                .write_all(format!("has_mcsvdl = {}\n",check_mcsvdl_win_pe()).as_bytes())
                .expect("Could not write to file");
            cfg_file
                .write_all(format!("mcsvdl_path = \"{}\"\n", mcsvdl_path.display()).as_bytes())
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
 
// The TESTS 💀
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sanity_check();
        assert_eq!(result, "This works");
    }
    #[test]
    fn config_path_exists() {
        let result = get_config_path();
        assert_eq!(result, home_dir().unwrap_or(fallback_path()).join(".mc-server-manager").join("config.toml"))
    }
}
