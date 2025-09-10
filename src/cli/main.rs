// SPDX-License-Identifier: GPL-3.0-or-later


use app_lib::*;
fn main() {
    init();
    println!("Hello World! CLI bin running.");
}

fn init() {
    println!("Checking for daemon...");
    if establish_connection() == true {
        println!("Connection with daemon established sucessfully!");
    }
    else {
        println!("Could not connet to daemon! Is the daemon running?");
    }
}