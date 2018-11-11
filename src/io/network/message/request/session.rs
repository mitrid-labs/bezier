use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::io::Permission;

use io::network::message::message::session::*;
use io::Request;

pub type SessionRequest = Request<Permission>;

pub fn check_session_req(req: &SessionRequest) -> Result<()> {
    req.check()?;

    check_session_msg(&req.message)
}