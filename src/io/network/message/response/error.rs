use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use io::network::message::message::error::*;
use io::Response;

pub type ErrorResponse = Response<String>;

pub fn check_error_res(res: &ErrorResponse) -> Result<()> {
    res.check()?;

    check_error_msg(&res.message)
}