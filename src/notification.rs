#![allow(unused)]

use std::fmt::Display;

use chrono::{DateTime, Local};
use iced::Task;

use crate::Message;

#[derive(Debug, Clone)]
pub struct Notification {
    pub level: Level,
    pub message: String,
    pub time: DateTime<Local>,
    retry_message: Option<&'static Message>
}

impl Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Notification { 
    pub fn new<T: ToString>(level: Level, message: T, time: DateTime<Local>) -> Self {
        Self { 
            level, 
            message: message.to_string(), 
            time, 
            retry_message: None
        }
    }

    pub fn new_with_retry<T: ToString>(level: Level, message: T, time: DateTime<Local>, retry_message: &'static Message) -> Self {
        Self { 
            level, 
            message: message.to_string(), 
            time, 
            retry_message: Some(retry_message)
        }
    }

    pub fn retry(&self) -> Result<Task<Message>, String> {
        match self.retry_message {
            Some(message) => Ok(Task::done(message.clone())),
            None => Err(format!("retry called on a non retryable notif:\n{} {:?}:\n{}", &self.time.format("%Y/%m/%d %H:%M:%S"), &self.level, &self))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Level {
    Notice,
    Warning,
    Error
}