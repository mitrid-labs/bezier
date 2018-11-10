use mitrid_core::model::BlockGraph as BlockGraphBase;

use crypto::Digest;
use model::blockgraph::BlockGraphPayload;

pub type BlockGraph = BlockGraphBase<Digest, BlockGraphPayload>;