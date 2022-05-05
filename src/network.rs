use std::net;
pub struct Profile {
    username: String,
    local_ip: net::Ipv4Addr
}

pub struct Server {
    local_ip: net::Ipv4Addr,
    clients: Vec<net::Ipv4Addr>,
    hosting_port: u8,
    tcp_listener: net::TcpListener
}

impl Server {
    pub fn new(hosting_port: u8) -> std::io::Result<Server> {
        Ok(Server {
            local_ip: net::Ipv4Addr::LOCALHOST,
            clients: Vec::new(),
            hosting_port,
            tcp_listener: net::TcpListener::bind(format!("128.0.0.1:{}", hosting_port))?

        })
    }

    pub fn start(self) -> std::io::Result<>{
        let mut init_data: [u8; 128];
        let mut connection: net::TcpStream;
        for stream in self.tcp_listener.incoming() {
            match stream{
                Ok(res) => connection = res,
                Err(_) => continue
            }
        }
    }
}

pub struct Client {

}