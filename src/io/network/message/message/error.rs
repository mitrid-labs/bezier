use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::io::Resource;

use io::network::message::message::message::*;

pub struct ErrorMessage;

impl ErrorMessage {
    pub fn verify(msg: &Message) -> Result<bool> {
        msg.check()?;

        if msg.resource != Resource::Error {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn check(msg: &Message) -> Result<()> {
        msg.check()?;

        if msg.resource != Resource::Error {
            return Err(format!("invalid resource"));
        }

        Ok(())
    }
}