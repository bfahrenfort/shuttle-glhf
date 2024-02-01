use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt::Display;

#[derive(Serialize, FromRow)]
pub struct Program {
    pub id: i32,
    pub program_name: String, // The API deals in absolutes (strings, not enums)
    pub doctype: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ProgramNew {
    pub program_name: String,
    pub doctype: String,
    pub url: Option<String>,
}

impl Display for ProgramNew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Program Name: {}\nDoctype: {}\nURL: {}",
            self.program_name,
            self.doctype,
            self.url.clone().unwrap_or("none".to_string())
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Queue {
    pub id: i32,
    pub program_name: String,
    pub doctype: String,
    pub url: Option<String>,
    pub request_type: String,
}

impl Display for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Program Name: {}\nDoctype: {}\nURL: {}\nRequest Type: {}",
            self.program_name,
            self.doctype,
            self.url.clone().unwrap_or("none".to_string()),
            self.request_type
        )
    }
}

#[derive(Deserialize, FromRow, Serialize)]
pub struct QueueNew {
    pub program_name: String,
    pub doctype: String,
    pub url: Option<String>,
    pub request_type: String,
}

#[derive(FromRow)]
pub struct Admin {
    pub id: i32,
    pub username: Option<String>,
    pub token: String,
}
