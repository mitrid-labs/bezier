use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use io::network::message::message::blockchain::*;
use io::network::message::response::response::*;

pub struct BlockChainResponse;

impl BlockChainResponse {
    pub fn verify_get(res: &Response) -> Result<bool> {
        res.check()?;

        BlockChainMessage::verify_get(&res.message)
    }

    pub fn check_get(res: &Response) -> Result<()> {
        res.check()?;

        BlockChainMessage::check_get(&res.message)
    }
}