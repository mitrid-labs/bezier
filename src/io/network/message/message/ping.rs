use mitrid_core::base::Result;
use mitrid_core::io::{Method, Resource};

use io::network::message::message::message::*;

pub struct PingMessage;

impl PingMessage {
    pub fn verify(msg: &Message) -> Result<bool> {
        verify_none_msg(msg, &Method::Ping, &Resource::None)
    }

    pub fn check(msg: &Message) -> Result<()> {
        check_none_msg(msg, &Method::Ping, &Resource::None)
    }
}