use mitrid_core::base::Result;

use crypto::Digest;
use io::network::message::message::blockchain::*;
use io::Request;

pub type GetBlockChainRequest = Request<Digest>;

pub fn check_get_bc_req(req: &GetBlockChainRequest) -> Result<()> {
    check_get_bc_msg(&req.message)
}