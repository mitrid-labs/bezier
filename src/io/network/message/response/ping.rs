use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use io::network::message::message::ping::*;
use io::network::message::response::response::*;

pub struct PingResponse;

impl PingResponse {
    pub fn verify(res: &Response) -> Result<bool> {
        res.check()?;

        PingMessage::verify(&res.message)
    }

    pub fn check(res: &Response) -> Result<()> {
        res.check()?;

        PingMessage::check(&res.message)
    }
}