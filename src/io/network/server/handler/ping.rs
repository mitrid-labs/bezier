use mitrid_core::base::Result;

use io::Store;
use io::network::message::request::ping::*;
use io::PingResponse;

pub fn ping(_store: &mut Store,
            request: &PingRequest)
    -> Result<PingResponse>
{
    check_ping_req(request)?;

    PingResponse::new(&request.message)
}