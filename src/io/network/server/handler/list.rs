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
use io::network::server::handler::error::*;

pub fn list(store: &mut Store,
            request: &Request)
    -> Result<Response>
{
    request.check()?;

    let list = if NodeRequest::verify_list(request)? {
        let (from, to, count, skip) = NodeRequest::parse_list(request)?;
        Node::store_list(store, from, to, count, skip)?;
    } else if UTxORequest::verify_list(request)? {
        let (from, to, count, skip) = UTxORequest::parse_list(request)?;
        UTxO::store_list(store, from, to, count, skip)?;
    } else {
        return error(store, request, "invalid resource");
    };

    let payload = list.to_bytes()?;

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