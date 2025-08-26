use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;
use app_lib::*;

fn main() {
    println!("[{}] Starting APPNAMEd...", get_time_hms());
    match TcpListener::bind(DAEMON_ADDR) {
        Ok(listener) => {
        println!("[{}] Listening on TCP {}...", get_time_hms(), DAEMON_ADDR);
        for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let mut buffer = [0; 512];

                    match stream.read(&mut buffer) {
                        Ok(n) if n == 0 => return,
                        Ok(n) => {
                            let input = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

                            let response = match input.as_str() {
                                "hello" => connection_established(),
                                "function2" => function2(),
                                _ => "Unknown command".to_string(),
                            };

                            let response_with_newline = response + "\n";
                            let _ = stream.write_all(response_with_newline.as_bytes());
                        }
                        Err(e) => eprintln!("[{}] Read error: {}",get_time_hms(), e),
                    }
                });
            }
            Err(e) => eprintln!("[{}] Connection failed: {}",get_time_hms(), e),
        }
    }
        }
        Err(_) => {
            eprintln!("[{}] Could not establish a TCP Listener {}!", get_time_hms(),DAEMON_ADDR);
            eprintln!("[{}] Exiting APPNAMEd...", get_time_hms());
            std::process::exit(1);
        }
    };

    
}

fn connection_established() -> String {
    println!("[{}] Client established a sucessfull connection.", get_time_hms());
    return "Connection established sucessfully!".to_string();
}

fn function2() -> String {
    "Hello from function2!".to_string()
}
