use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3040")?;
    // While the client has connected to the stream 
    // we want to continuously request them for the message
    // they would like to send.

    loop {
        println!("Please enter your message: ");
        let mut message = String::new();
        std::io::stdin().read_line(&mut message)?;


        if &message == "quit" {
            stream.
            break;
        }

        println!("String is: {}", message);
        stream.write(message.as_bytes())?;


    

    }

    Ok(())
}
