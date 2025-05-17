use std::io::{self, Write};
use std::fs;
use std::path::Path;
use std::process::Command;
use json::Json;

fn main() {
    let mut path;

    loop {
        let mut input = String::new();

        print!("Enter file path: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read path");

        path = input.trim_end().to_string();

        let filetype = Path::new(&path).extension().and_then(|ext| ext.to_str());

        if filetype == Some("json") {
            match fs::read_to_string(&path) {
                Ok(contents_string) => {
                    println!("File is JSON");
                    break;
                },
                Err(e) => {
                    println!("Failed to read file: {}", e);
                    continue;
                }
            }
        } else {
            println!("File is not JSON! Please enter a Path to a JSON file.");
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

}
