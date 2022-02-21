use serde::{Deserialize, Serialize};

use crate::thread::ThreadMessage;

#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub participants: Vec<String>,
    pub messages: Vec<ThreadMessage>,
}
