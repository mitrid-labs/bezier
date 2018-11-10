use mitrid_core::model::Block as BlockBase;

use crypto::{Digest, Proof};
use model::Amount;
use model::{InputPayload, OutputPayload, TransactionPayload};
use model::block::BlockPayload;

pub type Block = BlockBase<Digest, Amount, InputPayload, OutputPayload, TransactionPayload, BlockPayload, Proof>;