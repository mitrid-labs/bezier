use mitrid_core::base::Result;

use crypto::Digest;
use model::Block;
use io::network::message::message::block::*;
use io::Request;

pub type LookupBlockRequest = Request<Digest>;

pub fn check_lookup_block_req(req: &LookupBlockRequest) -> Result<()> {
    check_lookup_block_msg(&req.message)
}

pub type GetBlockRequest = Request<Digest>;

pub fn check_get_block_req(req: &GetBlockRequest) -> Result<()> {
    check_lookup_block_msg(&req.message)
}

pub type CreateBlockRequest = Request<Block>;

pub fn check_create_block_req(req: &CreateBlockRequest) -> Result<()> {
    check_create_block_msg(&req.message)
}