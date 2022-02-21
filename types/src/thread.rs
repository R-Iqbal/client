use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadMessage {
    pub user_id: String,
    pub message: String,
    pub room_id: String,
}
