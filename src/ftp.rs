use std::{
    str::from_utf8,
    io::{Write, Read},
    net::{TcpStream},
    process::Command,
    sync::Arc,
    sync::Mutex,
};

use regex::Regex;

#[path = "ftp_data.rs"] mod ftp_data;
use ftp_data::Ftp_data;

#[path = "credential.rs"] pub mod credential;
use credential::Credential;

pub struct Ftp{
    stream: TcpStream,
    credentials: Arc<Mutex<Vec<Credential>>>,
    ftp_data: Option<Ftp_data>,
}

const WELCOME:&[u8] = dotenv!("WELCOME").as_bytes();

impl Ftp{

    pub fn new(stream: TcpStream, credentials: Arc<Mutex<Vec<Credential>>>) -> Ftp {
        Ftp { stream: stream, credentials: credentials, ftp_data: None }
    }
    
    pub fn handle_connection(&mut self){
        let user:Option<String>;
        let pass:Option<String>;

        self.write(WELCOME);
        
        user = match self.read(){
            Some(t_user) => grep(t_user,r"USER (.*)\r"),
            None => None,
        };
    
        self.write(b"331 Please specify the password.");
    
        pass = match self.read(){
            Some(t_user) => grep(t_user,r"PASS (.*)\r"),
            None => None,
        };
    
        
        if !user.is_some() || !pass.is_some() {
            self.login_incorrect();
        }
        else{
            let user = user.unwrap();
            let pass = pass.unwrap();
            println!("{0} {1}",user,pass);
            let mut correct: bool = false;

            for credential in self.credentials.lock().unwrap().iter(){
                correct = credential.compare(&user,&pass);
                if correct{
                    break;
                }
            }
            if !correct{
                self.login_incorrect();
            }
            else{
                self.login_successful();

                for n in 1..10{
                    sleep("0.5");
                    let command_option = match self.read(){
                        Some(t_user) => grep(t_user,r"(.*)\r"),
                        None => None,
                    };
                    if command_option.is_some(){
                        let command = command_option.unwrap();
                        println!("{:?}",command);
                        if command == "QUIT"{
                            self.write(b"221 Goodbye.");
                        }
                        if command == "SYST"{
                            self.write(b"215 UNIX Type: L8");
                        }
                        if command == "FEAT"{
                            self.write(b"211-Features:");
                            self.write(b" EPRT");
                            self.write(b" EPSV");
                            self.write(b" MDTM");
                            self.write(b" PASV");
                            self.write(b" REST STREAM");
                            self.write(b" SIZE");
                            self.write(b" TVFS");
                            self.write(b" UTF8");
                            self.write(b"211 End");
                        }
                        if command == "EPSV"{
                            self.write(b"229 Entering Extended Passive Mode (|||4444|)");
                            self.ftp_data = Some(Ftp_data::new(21000));
                        }
                        if command == "LIST"{
                            self.write(b"150 Here comes the directory listing.");
                            if self.ftp_data.is_some(){
                                self.ftp_data.take().expect("ERROR").send(b"-rw-r--r--    1 1000     1000         1964 May 01 12:26 passwd");
                            }

                            self.write(b"226 Directory send OK.");
                        }
                        //if command == "SIZE test.jpg"{
                        //    self.write(b"213 952497");
                        //    self.read_trash();
                        //    self.write(b"229 Entering Extended Passive Mode (|||21000|)");
                        //    self.read_trash();
                        //    self.write(b"150 Opening BINARY mode data connection for test.jpg (952497 bytes).");
                        //    self.write(b"0000000000000000000000000000000000000000000000000000000000000000000000000000000");
                        //    self.write(b"226 Transfer complete.");
                        //    self.read_trash();
                        //    self.write(b"213 20221031170017");
                        //}
                    }
                }
            }
        }
    }
    
    
    fn login_incorrect(&mut self){
        self.write(b"530 Login incorrect.");
        self.read_trash();
        self.write(b"221 Goodbye.");
    }

    fn login_successful(&mut self){
        self.write(b"230 Login successful.");
    }
    
    
    
    
    #[allow(unused_must_use)]
    fn write(&mut self, txt:&[u8]){
        sleep(dotenv!("DELAY"));
        let mut text:Vec<u8> = Vec::from(txt);
        text.extend_from_slice(b"\r\n");

        self.stream.write(&text);
    }
    #[allow(unused_must_use)]
    fn read_trash(&mut self){
        self.stream.read(&mut [0; 0]);
    }
    
    #[allow(unused_must_use)]
    #[allow(unused_variables)]
    fn read(&mut self) -> Option<String>{
        let mut resp:[u8; 32] = [0;32];
        self.stream.read(&mut resp);
        let user = match from_utf8(&resp){
            Ok(user) => Some(user.to_string()),
            Err(error) => return None,
        };
        if user.is_some(){
            Some(user.unwrap())
        }
        else{
            return None;
        }
    }
}
pub fn grep(resp:String,reg:&str) -> Option<String>{
        
    let re = Regex::new(reg).unwrap(); 

    let cap = re.captures(&resp);
    
    if !cap.is_some(){
        return None;
    }
    let cap = cap.unwrap();
    if cap.len() != 2{
        return None;
    }
    Some((&cap[1]).to_string())
}

fn sleep(time:&str){
    let mut child = Command::new("sleep").arg(time).spawn().unwrap();
    child.wait().expect("Wait broken");
}