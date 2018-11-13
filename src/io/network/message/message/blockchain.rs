use mitrid_core::base::Result;
use mitrid_core::io::{Method, Resource};

use io::network::message::message::message::*;

pub struct BlockChainMessage;

impl BlockChainMessage {
    pub fn verify_get(msg: &Message) -> Result<bool> {
        verify_read_msg(msg, &Method::Get, &Resource::BlockGraph)
    }

    pub fn check_get(msg: &Message) -> Result<()> {
        check_read_msg(msg, &Method::Get, &Resource::BlockGraph)
    }
}