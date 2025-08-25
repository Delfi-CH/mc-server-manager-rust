use std::net::TcpListener;
use app_lib::*;
fn main() {

    println!("[{}] Starting daemon bin...", get_time_hms());
    
    let localhost = "127.0.0.1:";
    let port = "29900";
    let listener = TcpListener::bind(localhost.to_owned()+port).unwrap();
    println!("[{}] Listening on {}{}", get_time_hms(), localhost, port);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        
    }
    println!("[{}] Hello World! Daemon bin running.", get_time_hms());
}