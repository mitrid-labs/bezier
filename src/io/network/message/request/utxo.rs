use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use crypto::Digest;
use io::network::message::message::utxo::*;
use io::Request;

pub type CountUTxOsRequest = Request<(Option<Digest>, Option<Digest>)>;

pub fn check_count_utxos_req(req: &CountUTxOsRequest) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    check_count_utxos_msg(&req.message)
}

pub type ListUTxOsRequest = Request<(Option<Digest>, Option<Digest>, Option<u64>)>;

pub fn check_list_utxos_req(req: &ListUTxOsRequest) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    check_list_utxos_msg(&req.message)
}

pub type LookupUTxORequest = Request<Digest>;

pub fn check_lookup_utxo_req(req: &LookupUTxORequest) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    check_lookup_utxo_msg(&req.message)
}

pub type GetUTxORequest = Request<Digest>;

pub fn check_get_utxo_req(req: &GetUTxORequest) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    check_get_utxo_msg(&req.message)
}