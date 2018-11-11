use mitrid_core::base::Result;

use model::Transaction;
use io::network::message::message::transaction::*;
use io::Response;

pub type LookupTxResponse = Response<bool>;

pub fn check_lookup_tx_res(res: &LookupTxResponse) -> Result<()> {
    check_lookup_tx_msg(&res.message)
}

pub type GetTxResponse = Response<Transaction>;

pub fn check_get_tx_res(res: &GetTxResponse) -> Result<()> {
    check_get_tx_msg(&res.message)
}

pub type CreateTxResponse = Response<()>;

pub fn check_create_tx_res(res: &CreateTxResponse) -> Result<()> {
    check_create_tx_msg(&res.message)
}