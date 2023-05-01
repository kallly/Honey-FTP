#[macro_use]
extern crate dotenv_codegen;


use std::{
    net::TcpListener,
    thread::spawn,
};

mod ftp;
use ftp::Ftp;

#[path = "credential.rs"] mod credential;
use ftp::credential::Credential;

use std::fs;


fn main() {
    println!("In file {}", "allow.txt");

    let contents = fs::read_to_string("allow.txt")
        .expect("Should have been able to read the file");
    let mut credentials:Vec<Credential> = Vec::new();
    
    for line in contents.lines(){
        let username:String = ftp::grep(String::from(line),r"(.*):").unwrap();
        let password:String = ftp::grep(String::from(line),r":(.*)").unwrap();
        let credential:Credential = Credential::new(username,password);
        credentials.push(credential);
    }

    let listener = TcpListener::bind(dotenv!("LHOST_LPORT")).unwrap();

    for stream in listener.incoming() {
        spawn(||{
            let mut ftp = Ftp::new(stream.unwrap(),credentials);
            ftp.handle_connection();
        });
    }
}

