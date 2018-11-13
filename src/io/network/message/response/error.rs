use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use io::network::message::message::error::*;
use io::network::message::response::response::*;

pub struct ErrorResponse;

impl ErrorResponse {
    pub fn verify(res: &Response) -> Result<bool> {
        res.check()?;

        ErrorMessage::verify(&res.message)
    }

    pub fn check(res: &Response) -> Result<()> {
        res.check()?;

        ErrorMessage::check(&res.message)
    }
}