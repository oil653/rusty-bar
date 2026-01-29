use std::fmt::Display;

use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct Notification {
    pub level: Level,
    pub message: String,
    pub time: DateTime<Local>
}

impl Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Notification { 
    pub fn new<T: ToString>(level: Level, message: T, time: DateTime<Local>) -> Self {
        Self { level, message: message.to_string(), time }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Level {
    Notice,
    Warning,
    Error
}