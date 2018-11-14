use mitrid_core::io::Server as ServerBase;

use crypto::Digest;
use io::Store;
use io::Address;
use io::{ServerTransport, ClientTransport};
use io::{Handler, Router};

#[derive(Clone)]
pub struct Server;

impl ServerBase<Store, (), ServerTransport, ClientTransport, Address, Handler, Router, (), Digest, Vec<u8>> for Server {}