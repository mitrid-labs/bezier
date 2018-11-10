use mitrid_core::model::Transaction as TransactionBase;

use crypto::Digest;
use model::Amount;
use model::InputPayload;
use model::transaction::TransactionPayload;

pub type Transaction = TransactionBase<Digest, Amount, InputPayload, Digest, TransactionPayload>;