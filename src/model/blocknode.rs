use mitrid_core::model::BlockNode as BlockNodeBase;

use crypto::Digest;

pub type BlockNode = BlockNodeBase<Digest>;