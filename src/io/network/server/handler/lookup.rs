use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::io::Storable;

use crypto::Hasher;
use model::{UTxO, Transaction, Block};
use io::Node;
use io::Store;
use io::network::{Message, Request, Response};
use io::network::message::request::node::*;
use io::network::message::request::utxo::*;
use io::network::message::request::transaction::*;
use io::network::message::request::block::*;

pub fn lookup_handler(store: &mut Store,
                     request: &Request)
    -> Result<Response>
{
    request.check()?;

    let found = if NodeRequest::verify_lookup(request)? {
        let id = NodeRequest::parse_lookup(request)?;
        Node::store_lookup(store, &id)?;
    } else if UTxORequest::verify_lookup(request)? {
        let id = UTxORequest::parse_lookup(request)?;
        UTxO::store_lookup(store, &id)?;
    } else if TransactionRequest::verify_lookup(request)? {
        let id = TransactionRequest::parse_lookup(request)?;
        Transaction::store_lookup(store, &id)?;
    } else if BlockRequest::verify_lookup(request)? {
        let id = BlockRequest::parse_lookup(request)?;
        Block::store_lookup(store, &id)?;
    } else {
        return Err(format!("invalid request"));
    };

    let payload = found.to_bytes()?;

    let mut hasher = Hasher{};

    let message = Message::new()
                        .meta(&request.message.meta)?
                        .session(&request.message.session)?
                        .method(&request.message.method)?
                        .resource(&request.message.resource)?
                        .payload(&payload)?
                        .finalize(&mut hasher)?;

    Response::new(&message)
}