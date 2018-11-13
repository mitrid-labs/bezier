use mitrid_core::base::Result;

use io::Store;
use io::{Request, PingRequest, Response};

pub fn ping(_store: &mut Store,
            request: &Request)
    -> Result<Response>
{
    PingRequest::check(request)?;

    Response::new(&request.message)
}