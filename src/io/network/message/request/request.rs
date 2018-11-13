use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Datable;
use mitrid_core::io::Request as RequestBase;

use crypto::Digest;
use io::parse_msg;

pub type Request = RequestBase<(), Digest, Vec<u8>>;

pub fn check_req(req: &Request) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    Ok(())
}

pub fn parse_req<P: Datable + Serializable>(req: &Request) -> Result<P> {
    req.check()?;

    parse_msg(&req.message)
}