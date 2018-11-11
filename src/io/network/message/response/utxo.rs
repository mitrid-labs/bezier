use mitrid_core::base::Result;

use model::UTxO;
use io::network::message::message::utxo::*;
use io::Response;

pub type CountUTxOsResponse = Response<u64>;

pub fn check_count_utxos_res(res: &CountUTxOsResponse) -> Result<()> {
    check_count_utxos_msg(&res.message)
}

pub type ListUTxOsResponse = Response<u64>;

pub fn check_list_utxos_res(res: &ListUTxOsResponse) -> Result<()> {
    check_list_utxos_msg(&res.message)
}

pub type LookupUTxOResponse = Response<bool>;

pub fn check_lookup_utxo_res(res: &LookupUTxOResponse) -> Result<()> {
    check_lookup_utxo_msg(&res.message)
}

pub type GetUTxOResponse = Response<UTxO>;

pub fn check_get_utxo_res(res: &GetUTxOResponse) -> Result<()> {
    check_lookup_utxo_msg(&res.message)
}