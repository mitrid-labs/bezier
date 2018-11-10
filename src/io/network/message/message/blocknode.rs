use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Datable;
use mitrid_core::io::{Permission, Method, Resource};

use crypto::Digest;
use model::BlockNode;
use io::Message;

pub type LookupBlockNodeReqMsg = Message<Digest>;
pub type LookupBlockNodeResMsg = Message<bool>;

pub fn check_lookup_bn_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
    msg.check()?;

    if msg.session.is_expired()? {
        return Err(format!("expired session"));
    }

    if msg.session.permission > Permission::Read {
        return Err(format!("invalid permission"));
    }

    if msg.method != Method::Lookup {
        return Err(format!("invalid method"));
    }

    if msg.resource != Resource::BlockNode {
        return Err(format!("invalid resource"));
    }

    Ok(())
}

pub type GetBlockNodeReqMsg = Message<Digest>;
pub type GetBlockNodeResMsg = Message<BlockNode>;

pub fn check_get_bn_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
    msg.check()?;

    if msg.session.is_expired()? {
        return Err(format!("expired session"));
    }

    if msg.session.permission > Permission::Read {
        return Err(format!("invalid permission"));
    }

    if msg.method != Method::Get {
        return Err(format!("invalid method"));
    }

    if msg.resource != Resource::BlockNode {
        return Err(format!("invalid resource"));
    }

    Ok(())
}