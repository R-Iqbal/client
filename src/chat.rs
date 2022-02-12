use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use indicatif::ProgressBar;
use rand::rngs::OsRng;

use rsa::{RsaPrivateKey, RsaPublicKey};

use std::error::Error;

pub struct Cli {}

pub struct Client {
    pub username: String,
    pub connection: TcpStream,
}

impl Client {
    // Creates a new client which connects to the server
    pub fn new(username: String) -> Result<Client, Box<dyn Error>> {
        let mut connection = TcpStream::connect("127.0.0.1:3040")?;

        let string = format!("!username {}", username);
        connection.write(string.trim().as_bytes())?;
        Ok(Client {
            username,
            connection,
        })
    }
}
