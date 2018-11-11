use mitrid_core::base::Result;

use io::Session;
use io::network::message::message::session::*;
use io::Response;

pub type SessionResponse = Response<Session>;

pub fn check_session_res(res: &SessionResponse) -> Result<()> {
    check_session_msg(&res.message)
}