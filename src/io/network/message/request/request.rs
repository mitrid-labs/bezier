use mitrid_core::io::Request as RequestBase;

use crypto::Digest;
use io::Address;

pub type Request<P> = RequestBase<(), Address, (), Digest, P>;