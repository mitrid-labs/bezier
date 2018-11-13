use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::io::Request as RequestBase;

use crypto::Digest;

pub type Request = RequestBase<(), Digest, Vec<u8>>;

pub fn check_req(req: &Request) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    Ok(())
}