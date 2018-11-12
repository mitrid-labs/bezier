use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Datable;
use mitrid_core::io::Storable;
use mitrid_core::io::{Method, Resource};

use std::any::Any;

use crypto::Digest;
use model::UTxO;
use io::Node;
use io::Store;
use io::Address;
use io::{Request, Response};
use io::network::message::request::node::*;
//use io::network::message::response::node::*;
use io::network::message::request::utxo::*;
//use io::network::message::response::utxo::*;

pub fn count<P, R>(store: &mut Store,
                   request: &Request<P>)
    -> Result<Response<R>>
    where   P: Any + Datable,
            R: Datable
{
    request.check()?;

    if request.message.method != Method::Count {
        return Err(format!("invalid method"));
    }

    match request.message.resource {
        Resource::Node => {
            check_count_nodes_req(request as &CountNodesRequest)?;

            let count = Node::store_count(store, payload.0, payload.1)?;
            
            // TODO: build the response and return it

            unreachable!()

        },
        Resource::Coin => {
            check_count_utxos_req(request as &CountUTxOsRequest)?;
            
            let count = UTxO::store_count(store, payload.0, payload.1)?;
            
            // TODO: build the response and return it

            unreachable!()
        },
        _ => {
            Err(format!("invalid resource"))
        },
    }
}