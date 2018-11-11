use mitrid_core::base::Result;

use crypto::Digest;
use io::network::message::message::utxo::*;
use io::Request;

pub type CountUTxOsRequest = Request<(Option<Digest>, Option<Digest>)>;

pub fn check_count_utxos_req(req: &CountUTxOsRequest) -> Result<()> {
    check_count_utxos_msg(&req.message)
}

pub type ListUTxOsRequest = Request<(Option<Digest>, Option<Digest>, Option<u64>)>;

pub fn check_list_utxos_req(req: &ListUTxOsRequest) -> Result<()> {
    check_list_utxos_msg(&req.message)
}

pub type LookupUTxORequest = Request<Digest>;

pub fn check_lookup_utxo_req(req: &LookupUTxORequest) -> Result<()> {
    check_lookup_utxo_msg(&req.message)
}

pub type GetUTxORequest = Request<Digest>;

pub fn check_get_utxo_req(req: &GetUTxORequest) -> Result<()> {
    check_get_utxo_msg(&req.message)
}