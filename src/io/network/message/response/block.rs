use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Datable;
use mitrid_core::io::{Permission, Method, Resource};

use crypto::Digest;
use model::Block;
use io::Message;

pub type LookupTxReqMsg = Message<Digest>;
pub type LookupTxResMsg = Message<bool>;

pub fn check_lookup_block_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
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

    if msg.resource != Resource::Block {
        return Err(format!("invalid resource"));
    }

    Ok(())
}

pub type GetTxReqMsg = Message<Digest>;
pub type GetTxResMsg = Message<Block>;

pub fn check_get_block_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
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

    if msg.resource != Resource::Block {
        return Err(format!("invalid resource"));
    }

    Ok(())
}

pub type CreateTxReqMsg = Message<Block>;
pub type CreateTxResMsg = Message<()>;

pub fn check_create_block_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
    msg.check()?;

    if msg.session.is_expired()? {
        return Err(format!("expired session"));
    }

    if msg.session.permission < Permission::Write {
        return Err(format!("invalid permission"));
    }

    if msg.method != Method::Create {
        return Err(format!("invalid method"));
    }

    if msg.resource != Resource::Block {
        return Err(format!("invalid resource"));
    }

    Ok(())
}