use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Datable;
use mitrid_core::io::Response as ResponseBase;

use crypto::Digest;
use io::parse_msg;

pub type Response = ResponseBase<(), Digest, Vec<u8>>;

pub fn parse_res<P: Datable + Serializable>(res: &Response) -> Result<P> {
    res.check()?;

    parse_msg(&res.message)
}