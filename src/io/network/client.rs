use mitrid_core::base::Result;
use mitrid_core::base::Datable;
use mitrid_core::io::Client as ClientBase;

use crypto::Digest;
use io::Address;
use io::ClientTransport;
use io::Request;

#[derive(Clone)]
pub struct Client;

impl ClientBase<ClientTransport, Address, (), Digest, Vec<u8>> for Client {
    fn build<P: Datable>(&mut self, _params: &P, _address: &Address)
        -> Result<Vec<Request>>
    {
        unreachable!()
    }
}