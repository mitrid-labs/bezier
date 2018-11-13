use mitrid_core::base::Result;
use mitrid_core::io::{Method, Resource};

use io::network::message::message::message::*;

pub struct SessionMessage;

impl SessionMessage {
    pub fn verify(msg: &Message) -> Result<bool> {
        verify_read_msg(msg, &Method::Session, &Resource::Session)
    }

    pub fn check(msg: &Message) -> Result<()> {
        check_read_msg(msg, &Method::Session, &Resource::Session)
    }
}