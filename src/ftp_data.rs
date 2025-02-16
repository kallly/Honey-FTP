use std::{
    io::{Write},
    net::{TcpListener, TcpStream, SocketAddr, Ipv4Addr},
    env,
};

#[allow(dead_code)]
pub struct FtpData{
    addr: SocketAddr,
    listener: TcpListener,
    stream: TcpStream,
    client: SocketAddr,
}

impl FtpData{
    pub fn new(port:u16) -> FtpData {
        let ip_arr: Vec<u8> = env::var("LHOST").unwrap().split('.').into_iter().map(|x| x.parse::<u8>().unwrap()).collect();
        let ip = Ipv4Addr::new(ip_arr[0],ip_arr[1],ip_arr[2],ip_arr[3]);
        let addr = SocketAddr::from((ip, port));
        let listener = TcpListener::bind(addr).unwrap();
        
        let stream = listener.accept().unwrap();
        
        FtpData { addr: addr, listener: listener, stream: stream.0, client: stream.1}
    }
    
    #[allow(unused_must_use)]
    pub fn send(&mut self, txt:&[u8]){
        let mut text:Vec<u8> = Vec::from(txt);
        text.extend_from_slice(b"\r\n");
        self.stream.write(&text);
    }
    #[allow(unused_must_use)]
    pub fn sendnoend(&mut self, txt:&[u8]){
        self.stream.write(&txt);
    }
}