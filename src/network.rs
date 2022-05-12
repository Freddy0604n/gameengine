use std::net;
use std::thread;

pub struct Profile {
    username: String,
    local_ip: net::Ipv4Addr
}

pub struct Server {
    local_ip: net::Ipv4Addr,
    clients: Vec<thread::JoinHandle>,
    hosting_port: u8,
    tcp_listener: net::TcpListener,
}

impl Server {
    pub fn new(hosting_port: u8) -> std::io::Result<Server> {
        Ok(Server {
            local_ip: net::Ipv4Addr::LOCALHOST,
            clients: Vec::new(),
            hosting_port,
            tcp_listener: net::TcpListener::bind(format!("127.0.0.1:{}", hosting_port))?,
        })
    }
    pub fn start(self) {
        for i in 0..10 {
            for stream in self.tcp_listener.incoming() {
                self.clients.push(std::thread::spawn(move || {
                    
                }))
            }
        }
    }
}

pub struct Client {
    profile: Profile,
}