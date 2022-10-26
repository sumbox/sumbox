use std::env;
use std::net::{SocketAddr, ToSocketAddrs};

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub server_address: SocketAddr,
}

impl Config {
    pub fn new() -> Config {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let server_host_url: String = env::var("SERVER_URL").expect("SERVER_URL not set");
        // returns array of address["<ipv4>","<ipv6>"]
        let server: Vec<_> = server_host_url
            .to_socket_addrs()
            .expect("Unable to resolve domain")
            .collect();
        // user IPV4 address for now
        let server_address: SocketAddr = server[0];
        Config {
            database_url,
            server_address,
        }
    }
}
