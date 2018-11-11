use mitrid_core::base::Result;

use model::BlockChain;
use io::network::message::message::blockchain::*;
use io::Response;

pub type GetBlockChainResponse = Response<BlockChain>;

pub fn check_get_bc_res(res: &GetBlockChainResponse) -> Result<()> {
    check_get_bc_msg(&res.message)
}