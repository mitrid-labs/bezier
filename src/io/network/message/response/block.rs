use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use model::Block;
use io::network::message::message::block::*;
use io::Response;

pub type LookupTxResponse = Response<bool>;

pub fn check_lookup_block_res(res: &LookupTxResponse) -> Result<()> {
    res.check()?;

    check_lookup_block_msg(&res.message)
}

pub type GetTxResponse = Response<Block>;

pub fn check_get_block_res(res: &GetTxResponse) -> Result<()> {
    res.check()?;

    check_get_block_msg(&res.message)
}

pub type CreateTxResponse = Response<()>;

pub fn check_create_block_res(res: &CreateTxResponse) -> Result<()> {
    res.check()?;

    check_get_block_msg(&res.message)
}