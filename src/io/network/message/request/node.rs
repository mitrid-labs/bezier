use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use io::network::message::message::node::*;
use io::Address;
use io::Request;

pub type CountNodesRequest = Request<(Option<Address>, Option<Address>)>;

pub fn check_count_nodes_req(req: &CountNodesRequest) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    check_count_nodes_msg(&req.message)
}

pub type ListNodesRequest = Request<(Option<Address>, Option<Address>, Option<u64>)>;

pub fn check_list_nodes_req(req: &ListNodesRequest) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    check_list_nodes_msg(&req.message)
}

pub type LookupTxRequest = Request<Address>;

pub fn check_lookup_node_req(req: &LookupTxRequest) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    check_lookup_node_msg(&req.message)
}

pub type GetTxRequest = Request<Address>;

pub fn check_get_node_req(req: &GetTxRequest) -> Result<()> {
    req.check()?;

    if req.message.session.is_expired()? {
        return Err(format!("expired session"));
    }

    check_get_node_msg(&req.message)
}