use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Datable;
use mitrid_core::io::Storable;

use std::any::Any;

use model::UTxO;
use io::Node;
use io::Store;
use io::network::{Request, Response};
use io::network::message::request::node::*;
use io::network::message::response::node::*;
use io::network::message::request::utxo::*;
use io::network::message::response::utxo::*;

pub fn count_handler<P, R>(store: &mut Store,
                           request: &Request<P>)
    -> Result<Response<R>>
    where   P: Datable,
            R: Datable
{
    request.check()?;

    let req: &Any = request as &Any;

    if req.is::<CountNodesRequest>() {
        let req = req.downcast_ref::<CountNodesRequest>().unwrap();
        let res: Response<P> = count_nodes_handler(store, req)?;
        let res: &Any = res as &Any;
        let res = res.downcast_ref::<Response<R>>().unwrap()
    } else if req.is::<CountUTxOsRequest>() {
        // TODO
    } else {
        // TODO
    }

    unreachable!()
}

pub fn count_nodes_handler(store: &mut Store,
                           request: &CountNodesRequest)
    -> Result<CountNodesResponse>
{
    request.check()?;
    check_count_nodes_req(request)?;

    let payload = request.message.payload.clone();

    let _count = Node::store_count(store, payload.0, payload.1)?;
    
    // TODO: build the response and return it

    unreachable!()
}

pub fn count_utxos_handler(store: &mut Store,
                           request: &CountUTxOsRequest)
    -> Result<CountUTxOsResponse>
{
    request.check()?;
    check_count_utxos_req(request)?;

    let payload = request.message.payload.clone();

    let _count = UTxO::store_count(store, payload.0, payload.1)?;
    
    // TODO: build the response and return it

    unreachable!()
}