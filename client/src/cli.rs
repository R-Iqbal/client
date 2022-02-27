use crate::chat;
use crate::terminal;
use dialoguer::theme::ColorfulTheme;
use dialoguer::*;
use std::error::Error;
use std::io::Write;
use std::os::windows::prelude::MetadataExt;
use std::thread;

use types::socket::{SocketMessage, SocketPayloadKind};

use serde_json::{Deserializer, Value};

pub fn main() -> Result<(), Box<dyn Error>> {
    // Setup a new terminal context
    let terminal = terminal::Terminal::new()?;

    // Generate keypair and request the user's username
    let keypair = terminal.generate_keypair()?;
    let username = terminal.request_username()?;

    // Create a new client which will connect to the server

    let client = chat::Client::new(username, keypair)?;

    let values = Deserializer::from_reader(&client.connection).into_iter::<Value>();

    for value in values {
        let value = value.unwrap();
        let message: SocketMessage = serde_json::from_value(value).unwrap();

        match message.payload {
            SocketPayloadKind::Connected { username } => todo!(),
            SocketPayloadKind::SetUsername { user_id, username } => todo!(),
            SocketPayloadKind::Disconnected { username } => todo!(),
            SocketPayloadKind::CreateRoom { roomId } => todo!(),
            SocketPayloadKind::JoinRoom { userId, roomId } => todo!(),
            SocketPayloadKind::ListRooms => todo!(),
            SocketPayloadKind::Rooms { rooms } => {
                // Since the user wants to list the available rooms we will ask them which room they would like to join.
                let chosen = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Which room would like to join")
                    .items(&rooms)
                    .interact()?;

                let join_room_message = SocketMessage {
                    payload: SocketPayloadKind::JoinRoom {
                        userId: client.user_id.clone(),
                        roomId: rooms[chosen].clone(),
                    },
                };

                (&client.connection).write_all(&serde_json::to_vec(&join_room_message).unwrap())?;

                // let (x, y) = terminal.term.size();

                // terminal.term.move_cursor_to(0, y.into()).unwrap();
                let user_message = Input::<String>::with_theme(&ColorfulTheme::default())
                    .allow_empty(false)
                    .interact_text()
                    .unwrap();

                let socket_message = SocketMessage {
                    payload: SocketPayloadKind::Message {
                        userId: client.user_id.clone(),
                        roomId: rooms[chosen].clone(),
                        message: user_message,
                    },
                };

                (&client.connection)
                    .write_all(&serde_json::to_vec(&socket_message).unwrap())
                    .unwrap();
            }

            SocketPayloadKind::Connected { username } => todo!(),
            SocketPayloadKind::SetUsername { user_id, username } => todo!(),
            SocketPayloadKind::Disconnected { username } => todo!(),
            SocketPayloadKind::CreateRoom { roomId } => todo!(),
            SocketPayloadKind::JoinRoom { userId, roomId } => todo!(),
            SocketPayloadKind::ListRooms => todo!(),
            SocketPayloadKind::Message {
                userId,
                roomId,
                message,
            } => println!("{}: {}", userId, message),
            SocketPayloadKind::Rooms { rooms } => todo!(),
            SocketPayloadKind::Ack => println!("Server has acknowledged our connection!"),
        }
    }

    Ok(())
}
