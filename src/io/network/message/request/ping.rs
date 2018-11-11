use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use io::network::message::message::ping::*;
use io::Request;

pub type PingRequest = Request<()>;

pub fn check_ping_req(req: &PingRequest) -> Result<()> {
    req.check()?;

    check_ping_msg(&req.message)
}