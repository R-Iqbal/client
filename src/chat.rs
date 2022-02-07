use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub struct Client {
    pub username: String,
    pub connection: TcpStream,
}

impl Client {
    pub fn new(username: String) -> Result<Client, std::io::Error> {
        let mut connection = TcpStream::connect("127.0.0.1:3040")?;

        let string = format!("!username {}", username);

        connection.write(string.trim().as_bytes())?;
        Ok(Client {
            username,
            connection,
        })
    }
}
