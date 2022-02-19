use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct SocketMessage {
    pub payload: SocketPayloadKind,
}

#[derive(Serialize, Deserialize)]
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
    Message {
        userId: String,
        roomId: String,
        message: String,
    },
    Rooms {
        rooms: Vec<String>,
    },
}
