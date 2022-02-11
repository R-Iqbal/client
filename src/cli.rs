use std::error::Error;
use std::fmt::Alignment;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use colored::Colorize;

use dialoguer::theme::ColorfulTheme;
use dialoguer::*;

use console::Term;

use dialoguer::Input;

use console::style;
use indicatif::ProgressBar;
use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};

use sha2::{Digest, Sha256, Sha512};

use crate::terminal;

// let mut hasher = Sha256::new();
// hasher.update(b"hello world");
pub fn main() -> Result<(), Box<dyn Error>> {
    let x = terminal::Terminal::new()?;

    let username = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your username")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.len() >= 4 {
                Ok(())
            } else {
                Err("Please enter a username longer than 4 characters.")
            }
        })
        .interact_text()?;

    let items = vec!["Earth", "Jupiter"];
    let chosen = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which room would like to join")
        .item("Earth")
        .item("Mars")
        .item("Jupiter")
        .interact()?;

    println!("Attempting to join a new room {}", chosen);
    // let mut client = chat::Client::new(username)?;
    // println!("This is {} neat", style("quite").cyan());
    // loop {
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input)?;
    //     client.connection.write(input.trim().as_bytes()).unwrap();
    // }
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

        print!("12345");
        print!("{}", (8u8 as char));
    }

    Ok(username)
}
