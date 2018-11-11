use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use crypto::Digest;
use model::Transaction;
use io::network::message::message::transaction::*;
use io::Request;

pub type LookupTxRequest = Request<Digest>;

pub fn check_lookup_tx_req(req: &LookupTxRequest) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    check_lookup_tx_msg(&req.message)
}

pub type GetTxRequest = Request<Digest>;

pub fn check_get_tx_req(req: &GetTxRequest) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    check_get_tx_msg(&req.message)
}

pub type CreateTxRequest = Request<Transaction>;

pub fn check_create_tx_req(req: &CreateTxRequest) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    check_create_tx_msg(&req.message)
}