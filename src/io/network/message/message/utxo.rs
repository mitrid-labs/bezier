use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Datable;
use mitrid_core::io::{Permission, Method, Resource};

use crypto::Digest;
use model::UTxO;
use io::Message;

pub type CountUTxOsReqMsg = Message<(Option<Digest>, Option<Digest>)>;
pub type CountUTxOsResMsg = Message<u64>;

pub fn check_count_utxos_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
    msg.check()?;

    if msg.session.is_expired()? {
        return Err(format!("expired session"));
    }

    if msg.session.permission > Permission::Read {
        return Err(format!("invalid permission"));
    }

    if msg.method != Method::Count {
        return Err(format!("invalid method"));
    }

    if msg.resource != Resource::Coin {
        return Err(format!("invalid resource"));
    }

    Ok(())
}

pub type ListUTxOsReqMsg = Message<(Option<Digest>, Option<Digest>, Option<u64>)>;
pub type ListUTxOsResMsg = Message<u64>;

pub fn check_list_utxos_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
    msg.check()?;

    if msg.session.is_expired()? {
        return Err(format!("expired session"));
    }

    if msg.session.permission > Permission::Read {
        return Err(format!("invalid permission"));
    }

    if msg.method != Method::List {
        return Err(format!("invalid method"));
    }

    if msg.resource != Resource::Coin {
        return Err(format!("invalid resource"));
    }

    Ok(())
}

pub type LookupUTxOReqMsg = Message<Digest>;
pub type LookupUTxOResMsg = Message<bool>;

pub fn check_lookup_utxo_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
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

    if msg.resource != Resource::Coin {
        return Err(format!("invalid resource"));
    }

    Ok(())
}

pub type GetUTxOReqMsg = Message<Digest>;
pub type GetUTxOResMsg = Message<UTxO>;

pub fn check_get_utxo_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
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

    if msg.resource != Resource::Coin {
        return Err(format!("invalid resource"));
    }

    Ok(())
}