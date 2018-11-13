use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use io::Session;
use io::network::message::message::session::*;
use io::network::message::response::response::*;

pub struct SessionResponse;

impl SessionResponse {
    pub fn verify(res: &Response) -> Result<bool> {
        res.check()?;

        SessionMessage::verify(&res.message)
    }

    pub fn check(res: &Response) -> Result<()> {
        res.check()?;

        SessionMessage::check(&res.message)
    }

    pub fn parse(res: &Response) -> Result<Session> {
        parse_res(res)
    }
}