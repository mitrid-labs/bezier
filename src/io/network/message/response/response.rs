use mitrid_core::io::Response as ResponseBase;

use crypto::Digest;

pub type Response = ResponseBase<(), Digest, Vec<u8>>;