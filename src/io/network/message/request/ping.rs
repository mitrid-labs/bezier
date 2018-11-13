use mitrid_core::base::Result;

use io::network::message::message::ping::*;
use io::network::message::request::request::*;

pub struct PingRequest;

impl PingRequest {
    pub fn verify(req: &Request) -> Result<bool> {
        check_req(req)?;

        PingMessage::verify(&req.message)
    }

    pub fn check(req: &Request) -> Result<()> {
        check_req(req)?;

        PingMessage::check(&req.message)
    }
}