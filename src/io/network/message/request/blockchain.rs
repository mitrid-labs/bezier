use mitrid_core::base::Result;

use crypto::Digest;
use io::network::message::message::blockchain::*;
use io::network::message::request::request::*;

pub struct BlockChainRequest;

impl BlockChainRequest {
    pub fn verify_get(req: &Request) -> Result<bool> {
        check_req(req)?;

        BlockChainMessage::verify_get(&req.message)
    }

    pub fn check_get(req: &Request) -> Result<()> {
        check_req(req)?;

        BlockChainMessage::check_get(&req.message)
    }

    pub fn parse_get(req: &Request) -> Result<Digest> {
        parse_req(req)
    }
}