use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use io::network::message::message::ping::*;
use io::Response;

pub type PingResponse = Response<()>;

pub fn check_ping_res(res: &PingResponse) -> Result<()> {
    res.check()?;

    check_ping_msg(&res.message)
}