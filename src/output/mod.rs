pub mod error;
pub mod jot_display;
pub mod message;

use std::fmt::Display;

pub use error::{Error, JotResult};
pub use jot_display::*;
pub use message::Message;

pub enum Output {
    Message(Message),
    #[allow(unused)]
    Error(Error),
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Output::Message(msg) => msg.to_string(),
                Output::Error(err) => format!("\x1b[0;31merror\x1b[0m: {}", err),
            }
        )
    }
}
