use mitrid_core::base::Result;

use io::network::message::message::ping::*;
use io::Response;

pub type PingResponse = Response<()>;

pub fn check_ping_res(res: &PingResponse) -> Result<()> {
    check_ping_msg(&res.message)
}