use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:29900").unwrap();

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
                        Err(e) => eprintln!("Read error: {}", e),
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn connection_established() -> String {
    "Connection established sucessfully!".to_string()
}

fn function2() -> String {
    "Hello from function2!".to_string()
}
