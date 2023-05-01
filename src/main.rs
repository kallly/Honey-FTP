#[macro_use]
extern crate dotenv_codegen;


use std::{
    net::{TcpListener, SocketAddr, Ipv4Addr},
    thread::spawn,
    sync::Arc,
    sync::Mutex,
};
mod ftp;
use ftp::Ftp;

#[path = "credential.rs"] mod credential;
use ftp::credential::Credential;

use std::fs;


fn main() {

    let contents = fs::read_to_string("allow.txt")
        .expect("Should have been able to read the file");
    let mut credentials: Vec<Credential> = Vec::new();
    
    for line in contents.lines(){
        let username:String = ftp::grep(String::from(line),r"(.*):").unwrap();
        let password:String = ftp::grep(String::from(line),r":(.*)").unwrap();
        let credential:Credential = Credential::new(username,password);
        credentials.push(credential);
    }

    let l_credentials: Arc<Mutex<Vec<Credential>>> = Arc::new(Mutex::new(credentials));

    let ip_arr: Vec<u8> = dotenv!("LHOST").split('.').into_iter().map(|x| x.parse::<u8>().unwrap()).collect();
    let ip = Ipv4Addr::new(ip_arr[0],ip_arr[1],ip_arr[2],ip_arr[3]);
    let port = dotenv!("LPORT").parse::<u16>().unwrap();
    let addr = SocketAddr::from((ip, port));
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let clone_creds = l_credentials.clone();
        spawn(||{
            let mut ftp = Ftp::new(stream.unwrap(),clone_creds);
            ftp.handle_connection();
        });
    }
}

