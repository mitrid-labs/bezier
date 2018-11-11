use mitrid_core::base::Result;

use io::network::message::message::node::*;
use io::Address;
use io::Request;

pub type CountNodesRequest = Request<(Option<Address>, Option<Address>)>;

pub fn check_count_nodes_req(req: &CountNodesRequest) -> Result<()> {
    check_count_nodes_msg(&req.message)
}

pub type ListNodesRequest = Request<(Option<Address>, Option<Address>, Option<u64>)>;

pub fn check_list_nodes_req(req: &ListNodesRequest) -> Result<()> {
    check_list_nodes_msg(&req.message)
}

pub type LookupTxRequest = Request<Address>;

pub fn check_lookup_node_req(req: &LookupTxRequest) -> Result<()> {
    check_lookup_node_msg(&req.message)
}

pub type GetTxRequest = Request<Address>;

pub fn check_get_node_req(req: &GetTxRequest) -> Result<()> {
    check_get_node_msg(&req.message)
}