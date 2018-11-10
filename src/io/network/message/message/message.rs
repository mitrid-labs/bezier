use mitrid_core::io::Message as MessageBase;

use crypto::Digest;
use io::Address;

pub type Message<P> = MessageBase<(), Address, (), Digest, P>;