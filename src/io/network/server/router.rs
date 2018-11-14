use mitrid_core::io::Router as RouterBase;

use crypto::Digest;
use io::Store;
use io::Handler;

#[derive(Clone)]
pub struct Router;

impl RouterBase<Store, (), (), Digest, Vec<u8>, Handler> for Router {}