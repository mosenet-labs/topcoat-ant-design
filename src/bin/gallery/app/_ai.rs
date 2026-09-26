mod bubble;
pub(in crate::app) mod chat;
pub(super) mod extras;
mod message_list;
mod sender;

pub(in crate::app) use bubble::bubble_page;
pub(in crate::app) use chat::{chat_new_page, chat_notes_page, chat_page};
pub(in crate::app) use message_list::message_list_page;
pub(in crate::app) use sender::sender_page;
