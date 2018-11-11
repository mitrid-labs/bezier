use mitrid_core::io::Response as ResponseBase;

use crypto::Digest;

pub type Response<P> = ResponseBase<(), Digest, P>;