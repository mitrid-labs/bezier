use mitrid_core::model::BlockGraph as BlockGraphBase;

use crypto::Digest;
use model::blockchain::BlockGraphPayload;

pub type BlockChain = BlockGraphBase<Digest, BlockGraphPayload>;