use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;

use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use crossbeam_channel::{bounded, Receiver, Sender};
use crossbeam_utils::thread;

use serde_json::{Deserializer, Value};
use types::socket::{SocketMessage, SocketPayloadKind, SERVER_ADDRESS, SERVER_PORT};
use types::thread::ThreadMessage;
use types::Room;

pub struct Server {
    pub host: String,
    listener: TcpListener,
    pub connected_clients: u64,
    pub clients: Arc<Mutex<HashMap<String, String>>>,
    pub rooms: Arc<Mutex<HashMap<String, Room>>>,
}

impl Server {
    pub fn new() -> Result<Server, Box<dyn Error>> {
        let host = format!("{}:{}", SERVER_ADDRESS, SERVER_PORT);

        let listener = TcpListener::bind(&host).unwrap();

        let mut rooms_map = HashMap::<String, Room>::new();
        rooms_map.insert(
            String::from("Earth"),
            Room {
                participants: Vec::<String>::default(),
                messages: Vec::<ThreadMessage>::default(),
            },
        );

        let clients = Arc::new(Mutex::new(HashMap::<String, String>::new()));
        let rooms = Arc::new(Mutex::new(rooms_map));
        Ok(Server {
            host,
            listener,
            clients,
            rooms,
            connected_clients: 0,
        })
    }

    pub fn connection_handler(
        mut connected_clients: Arc<Mutex<HashMap<String, String>>>,
        mut rooms: Arc<Mutex<HashMap<String, Room>>>,
        mut stream: TcpStream,
        rx: Receiver<String>,
        tx: Sender<String>,
    ) {
        thread::scope(|s| {
            s.spawn(|s| {
                s.spawn(|_| {
                    // Collect all the messages from the channel
                    // Block until the sender has been dropped
                    rx.iter().for_each(|broadcast_message| {
                        // Try to parse the actual type from the message
                        let serialized_message: ThreadMessage =
                            serde_json::from_str(&broadcast_message).unwrap();

                        let socket_message: SocketMessage = SocketMessage {
                            payload: SocketPayloadKind::Message {
                                userId: serialized_message.user_id,
                                roomId: serialized_message.room_id,
                                message: serialized_message.message,
                            },
                        };
                        // Create a message to send to the client
                        let serialized = &serde_json::to_vec(&socket_message).unwrap();

                        (&stream).write_all(&serialized).unwrap();
                    });
                });
            });

            s.spawn(|s| {
                // Attempt to deserialize the incoming data from the stream
                let values = Deserializer::from_reader(&stream).into_iter::<Value>();
                for value in values {
                    let value = value.unwrap();

                    // Parse the message into a SocketMessage so we can determine
                    // how to handle the execution
                    let message: SocketMessage = serde_json::from_value(value).unwrap();

                    println!("Received a message from the client: {:?}", message);
                    // Examine the type of SocketPayload to determine how we should handle
                    // the request
                    match message.payload {
                        SocketPayloadKind::Connected { username } => todo!(),
                        SocketPayloadKind::SetUsername { user_id, username } => {
                            // Attempt to acquire the lock on the mutex
                            let mut connected_clients = connected_clients.lock().unwrap();
                            connected_clients.insert(user_id, username);
                        }
                        SocketPayloadKind::Disconnected { username } => todo!(),
                        SocketPayloadKind::CreateRoom { roomId } => todo!(),
                        SocketPayloadKind::JoinRoom { userId, roomId } => {
                            // Attempt to acquire the lock on the mutex
                            let mut rooms = rooms.lock().unwrap();

                            // Fetch the room to update the participants
                            let room = rooms.get_mut(&roomId).unwrap();
                            room.participants.push(userId);
                        }
                        SocketPayloadKind::ListRooms => {
                            // Attempt to acquire the lock on the mutex
                            let mut rooms = rooms.lock().unwrap();

                            // Return the list of room identifiers back to the client
                            let room_ids = rooms.keys().cloned().collect();

                            let message = SocketMessage {
                                payload: SocketPayloadKind::Rooms { rooms: room_ids },
                            };

                            let serialzed = &serde_json::to_vec(&message).unwrap();

                            (&stream).write_all(&serialzed).unwrap();
                        }
                        SocketPayloadKind::Rooms { rooms } => todo!(),
                        SocketPayloadKind::Message {
                            userId,
                            roomId,
                            message,
                        } => {
                            // Attempt to acquire the lock on the mutex
                            let mut rooms = rooms.lock().unwrap();

                            // Fetch the room
                            let room = rooms.get_mut(&roomId).unwrap();

                            let message = ThreadMessage {
                                user_id: userId,
                                message,
                                room_id: roomId,
                            };

                            tx.send(serde_json::to_string(&message).unwrap()).unwrap();

                            // Add the message to the room
                            room.messages.push(message);
                        }
                        SocketPayloadKind::Ack => todo!(),
                    }
                }
            });
        })
        .unwrap();
    }

    pub fn start_listening(&mut self) {
        let (s1, r1) = bounded::<String>(20);

        // For each of the incoming connections create a connection handler
        // to deal with any requests

        for stream in self.listener.incoming() {
            println!("A new client has connected!");
            let stream = stream.unwrap();

            self.connected_clients += 1;
            println!(
                "A new client has connected! There are now {} connected clinets",
                self.connected_clients
            );

            let clients_arc = Arc::clone(&self.clients);
            let rooms_arc = Arc::clone(&self.rooms);

            let (s2, r2) = (s1.clone(), r1.clone());

            Self::connection_handler(clients_arc, rooms_arc, stream, r2, s2);
        }
    }
}
