// SPDX-License-Identifier: GPL-3.0-or-later


use app_lib::*;
fn main() {
    init();
    println!("Hello World! CLI bin running.");

    //debug only
    download_jdk("linux".to_owned(), 8, "tar.gz".to_owned());
    println!();
    download_jdk("linux".to_owned(), 17, "tar.gz".to_owned());
    println!();
    download_jdk("linux".to_owned(), 21, "tar.gz".to_owned());
    println!();
    download_jdk("windows".to_owned(), 8, "zip".to_owned());
    println!();
    download_jdk("windows".to_owned(), 17, "zip".to_owned());
    println!();
    download_jdk("windows".to_owned(), 21, "zip".to_owned());
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