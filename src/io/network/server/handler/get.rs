use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::io::Storable;

use crypto::Hasher;
use model::{UTxO, Transaction, Block, BlockChain};
use io::Node;
use io::Store;
use io::network::{Message, Request, Response};
use io::network::message::request::node::*;
use io::network::message::request::utxo::*;
use io::network::message::request::transaction::*;
use io::network::message::request::block::*;
use io::network::message::request::blockchain::*;
use io::network::server::handler::error::*;

pub fn get(store: &mut Store,
           request: &Request)
    -> Result<Response>
{
    request.check()?;

    let payload = if NodeRequest::verify_get(request)? {
        let id = NodeRequest::parse_get(request)?;
        Node::store_get(store, &id)?.to_bytes()?
    } else if UTxORequest::verify_get(request)? {
        let id = UTxORequest::parse_get(request)?;
        UTxO::store_get(store, &id)?.to_bytes()?
    } else if TransactionRequest::verify_get(request)? {
        let id = TransactionRequest::parse_get(request)?;
        Transaction::store_get(store, &id)?.to_bytes()?
    } else if BlockRequest::verify_get(request)? {
        let id = BlockRequest::parse_get(request)?;
        Block::store_get(store, &id)?.to_bytes()?
    } else if BlockChainRequest::verify_get(request)? {
        let id = BlockChainRequest::parse_get(request)?;
        BlockChain::store_get(store, &id)?.to_bytes()?
    } else {
        return error(store, request, "invalid resource");
    };

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