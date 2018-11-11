use mitrid_core::io::Response as ResponseBase;

use crypto::Digest;
use io::Address;

pub type Response<P> = ResponseBase<(), Address, (), Digest, P>;