use mitrid_core::io::Client as ClientBase;

use crypto::Digest;
use io::Address;
use io::ClientTransport;

#[derive(Clone)]
pub struct Client;

impl ClientBase<ClientTransport, Address, (), Digest, Vec<u8>> for Client {}