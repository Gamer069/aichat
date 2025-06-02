// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Role {
    User,
    Ai
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => {
                f.write_str("User")
            },
            Role::Ai => {
                f.write_str("Ai")
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub uuid: Uuid,
}

#[tauri::command]
fn process_message(message: Message) -> Message {
    Message { role: Role::Ai, content: "something".to_owned(), uuid: Uuid::now_v7()  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![process_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
