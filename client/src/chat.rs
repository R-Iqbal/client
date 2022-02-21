use crate::terminal::Keypair;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use types::socket::{SocketMessage, SocketPayloadKind, SERVER_ADDRESS, SERVER_PORT};
pub struct Client {
    pub user_id: String,
    pub username: String,
    pub connection: TcpStream,
    pub keypair: Keypair,
}

impl Client {
    // Creates a new client which connects to the server
    pub fn new(username: String, keypair: Keypair) -> Result<Client, Box<dyn Error>> {
        let host = format!("{}:{}", SERVER_ADDRESS, SERVER_PORT);
        let mut connection = TcpStream::connect(host)?;

        // Convert the public key to a PEM format
        let public_key_pem = pkcs1::ToRsaPublicKey::to_pkcs1_pem(&keypair.public_key)?;

        // Encrypt the PEMified public key using SHA256 to generate a userId
        let mut hasher = Sha256::new();
        hasher.update(&public_key_pem);
        let user_id = format!("{:x}", hasher.finalize());

        // Create a new message to inform the server a new user
        // has registered and wants to est their username
        let set_username_message = SocketMessage {
            payload: SocketPayloadKind::SetUsername {
                username: username.clone(),
                user_id: user_id.clone(),
            },
        };

        let list_rooms_message = SocketMessage {
            payload: SocketPayloadKind::ListRooms,
        };

        connection.write_all(&serde_json::to_vec(&set_username_message).unwrap())?;
        connection.write_all(&serde_json::to_vec(&list_rooms_message).unwrap())?;

        // Return client instance
        Ok(Client {
            user_id,
            username,
            connection,
            keypair,
        })
    }
}
