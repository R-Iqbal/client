use crate::chat;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut server = chat::Server::new()?;

    server.start_listening();

    Ok(())
}
