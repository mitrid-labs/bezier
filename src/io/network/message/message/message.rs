use mitrid_core::io::Message as MessageBase;

use crypto::Digest;

pub type Message<P> = MessageBase<(), Digest, P>;