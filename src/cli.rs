use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use colored::Colorize;

use crate::chat;

pub fn main() -> std::io::Result<()> {
    let mut username = request_username()?;

    let mut client = chat::Client::new(username)?;

    Ok(())
}

fn request_username() -> Result<String, std::io::Error> {
    let mut username = String::new();
    println!("Please enter your username: ");
    std::io::stdin().read_line(&mut username)?;
    while username.trim().len() == 0 {
        println!("{}", "No username given.".red());
        println!("Please enter your username: ");
        std::io::stdin().read_line(&mut username)?;
    }

    Ok(username)
}
