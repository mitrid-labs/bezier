use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use io::Node;
use io::network::message::message::node::*;
use io::Response;

pub type CountNodesResponse = Response<u64>;

pub fn check_count_nodes_res(res: &CountNodesResponse) -> Result<()> {
    res.check()?;

    check_count_nodes_msg(&res.message)
}

pub type ListNodesResponse = Response<u64>;

pub fn check_list_nodes_res(res: &ListNodesResponse) -> Result<()> {
    res.check()?;

    check_list_nodes_msg(&res.message)
}

pub type LookupTxResponse = Response<bool>;

pub fn check_lookup_node_res(res: &LookupTxResponse) -> Result<()> {
    res.check()?;

    check_lookup_node_msg(&res.message)
}

pub type GetTxResponse = Response<Node>;

pub fn check_get_node_res(res: &GetTxResponse) -> Result<()> {
    res.check()?;

    check_lookup_node_msg(&res.message)
}