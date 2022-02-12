use crate::chat;
use crate::terminal;
use dialoguer::theme::ColorfulTheme;
use dialoguer::*;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use sha2::digest::XofReader;
use sha2::{Digest, Sha256, Sha512};
use std::error::Error;
use std::io::Read;

#[derive(Serialize, Deserialize)]

struct SocketMessage {
    r#type: String,
    data: DataKind,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum DataKind {
    StringVector(Vec<String>),
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // Setup a new terminal context
    let terminal = terminal::Terminal::new()?;

    // Generate keypair and request the user's username
    let keypair = terminal.generate_keypair()?;
    let username = terminal.request_username()?;

    // Create a new client which will connect to the server
    let mut client = chat::Client::new(username, keypair)?;

    // Create a byte buffer to read in the data from the server
    let mut data = [0 as u8; 200]; // using 50 byte buffer

    while match client.connection.read(&mut data) {
        Ok(size) => {
            // Read in the latest message from the socket
            let data = data[..size].to_vec();
            let message = String::from_utf8(data).unwrap();

            // Parse the message to determine what we have just recieved

            println!("Message is: {}", message);

            let parsed_message: SocketMessage = serde_json::from_str(&message).unwrap();

            match parsed_message.data {
                DataKind::StringVector(data) => {
                    let chosen = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Which room would like to join")
                        .items(&data)
                        .interact()?;
                }
            }

            println!("[Server]: {}", message);

            true
        }
        Err(_) => {
            println!("Oh, it looks like something went wrong!");
            true
        }
    } {}

    Ok(())
}
