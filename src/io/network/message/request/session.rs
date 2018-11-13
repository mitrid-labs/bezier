use mitrid_core::base::Result;

use io::network::message::message::session::*;
use io::network::message::request::request::*;

pub struct SessionRequest;

impl SessionRequest {
    pub fn verify(req: &Request) -> Result<bool> {
        check_req(req)?;

        SessionMessage::verify(&req.message)
    }

    pub fn check(req: &Request) -> Result<()> {
        check_req(req)?;

        SessionMessage::check(&req.message)
    }
}