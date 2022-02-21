pub const SERVER_ADDRESS: &str = "127.0.0.1";
pub const SERVER_PORT: &str = "3040";

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct SocketMessage {
    pub payload: SocketPayloadKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SocketPayloadKind {
    Ack,
    Connected {
        username: String,
    },
    SetUsername {
        user_id: String,
        username: String,
    },
    Disconnected {
        username: String,
    },
    CreateRoom {
        roomId: String,
    },
    JoinRoom {
        userId: String,
        roomId: String,
    },
    ListRooms,
    Rooms {
        rooms: Vec<String>,
    },
    Message {
        userId: String,
        roomId: String,
        message: String,
    },
}
