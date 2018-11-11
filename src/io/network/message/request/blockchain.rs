use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use crypto::Digest;
use io::network::message::message::blockchain::*;
use io::Request;

pub type GetBlockChainRequest = Request<Digest>;

pub fn check_get_bc_req(req: &GetBlockChainRequest) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    check_get_bc_msg(&req.message)
}