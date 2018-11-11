use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Datable;
use mitrid_core::io::{Permission, Method, Resource};

use crypto::Digest;
use model::BlockChain;
use io::Message;

pub type GetBlockChainReqMsg = Message<Digest>;
pub type GetBlockChainResMsg = Message<BlockChain>;

pub fn check_get_bc_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
    msg.check()?;

    if msg.session.permission > Permission::Read {
        return Err(format!("invalid permission"));
    }

    if msg.method != Method::Get {
        return Err(format!("invalid method"));
    }

    if msg.resource != Resource::BlockGraph {
        return Err(format!("invalid resource"));
    }

    Ok(())
}