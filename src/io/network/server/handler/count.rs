use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::io::Storable;

use crypto::Hasher;
use model::UTxO;
use io::Node;
use io::Store;
use io::network::{Message, Request, Response};
use io::network::message::request::node::*;
use io::network::message::request::utxo::*;

pub fn count_handler(store: &mut Store,
                     request: &Request)
    -> Result<Response>
{
    request.check()?;

    let count = if NodeRequest::verify_count(request)? {
        let (from, to) = NodeRequest::parse_count(request)?;
        Node::store_count(store, from, to)?;
    } else if UTxORequest::verify_count(request)? {
        let (from, to) = UTxORequest::parse_count(request)?;
        UTxO::store_count(store, from, to)?;
    } else {
        return Err(format!("invalid request"));
    };

    let payload = count.to_bytes()?;

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