use mitrid_core::io::Node as NodeBase;

use io::network::Address;
use io::network::node::NodePayload;

pub type Node = NodeBase<Address, NodePayload>;