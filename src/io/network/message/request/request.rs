use mitrid_core::io::Request as RequestBase;

use crypto::Digest;

pub type Request<P> = RequestBase<(), Digest, P>;