use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use model::UTxO;
use io::network::message::message::utxo::*;
use io::Response;

pub type CountUTxOsResponse = Response<u64>;

pub fn check_count_utxos_res(res: &CountUTxOsResponse) -> Result<()> {
    res.check()?;

    check_count_utxos_msg(&res.message)
}

pub type ListUTxOsResponse = Response<u64>;

pub fn check_list_utxos_res(res: &ListUTxOsResponse) -> Result<()> {
    res.check()?;

    check_list_utxos_msg(&res.message)
}

pub type LookupUTxOResponse = Response<bool>;

pub fn check_lookup_utxo_res(res: &LookupUTxOResponse) -> Result<()> {
    res.check()?;

    check_lookup_utxo_msg(&res.message)
}

pub type GetUTxOResponse = Response<UTxO>;

pub fn check_get_utxo_res(res: &GetUTxOResponse) -> Result<()> {
    res.check()?;

    check_lookup_utxo_msg(&res.message)
}