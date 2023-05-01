use std::{
    str::from_utf8,
    io::{Write, Read},
    net::{TcpStream},
    process::Command,
};
use regex::Regex;

#[path = "credential.rs"] pub mod credential;
use credential::Credential;

pub struct Ftp{
    stream: TcpStream,
    credentials: Vec<Credential>,
}

const WELCOME:&[u8] = dotenv!("WELCOME").as_bytes();

impl Ftp{

    pub fn new(stream:TcpStream, credentials:Vec<Credential>) -> Ftp {
        Ftp { stream: stream, credentials: credentials }
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
            if user != "anonymous" || pass != "" {
                self.login_incorrect();
            }
            else{
                self.login_successful();

                //while true{
                //    let command_option = match self.read(){
                //        Some(t_user) => grep(t_user,r"(.*)\r"),
                //        None => None,
                //    };
                //    if command_option.is_some(){
                //        let command = command_option.unwrap();
                //        println!("{:?}",command);
                //        if command == "EPSV"{
                //            self.write(b"229 Entering Extended Passive Mode (|||21000|)");
                //        }
                        //if command == "LIST"{
                        //    self.write(b"150 Here comes the directory listing.");
                        //    self.write(STRUCT);
                        //    self.write(b"226 Directory send OK.");
                        //}
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
                //    }
                //}
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
        self.read_trash();
        self.write(b"215 UNIX Type: L8");
        self.read_trash();
        self.write(b"211-Features:");
        self.write(b" EPRT");
        self.write(b" EPSV");
        //read(&stream,&mut [0; 4]);
        self.write(b" MDTM");
        self.write(b" PASV");
        //read(&stream,&mut [0; 4]);
        self.write(b" REST STREAM");
        self.write(b" SIZE");
        self.write(b" TVFS");
        self.read_trash();
        self.write(b" UTF8");
        self.write(b"211 End");
    }
    
    
    
    
    #[allow(unused_must_use)]
    fn write(&mut self, txt:&[u8]){
        sleep(dotenv!("DELAY"));
        let mut text:Vec<u8> = Vec::from(txt);
        text.extend_from_slice(b"\r\n");

        //temp = temp + b"";
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