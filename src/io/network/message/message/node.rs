use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Datable;
use mitrid_core::io::{Permission, Method, Resource};

use io::{Address, Node};
use io::Message;

pub type CountNodesReqMsg = Message<(Option<Address>, Option<Address>)>;
pub type CountNodesResMsg = Message<u64>;

pub fn check_count_nodes_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
    msg.check()?;

    if msg.session.permission > Permission::Read {
        return Err(format!("invalid permission"));
    }

    if msg.method != Method::Count {
        return Err(format!("invalid method"));
    }

    if msg.resource != Resource::Node {
        return Err(format!("invalid resource"));
    }

    Ok(())
}

pub type ListNodesReqMsg = Message<(Option<Address>, Option<Address>, Option<u64>)>;
pub type ListNodesResMsg = Message<u64>;

pub fn check_list_nodes_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
    msg.check()?;

    if msg.session.permission > Permission::Read {
        return Err(format!("invalid permission"));
    }

    if msg.method != Method::List {
        return Err(format!("invalid method"));
    }

    if msg.resource != Resource::Node {
        return Err(format!("invalid resource"));
    }

    Ok(())
}

pub type LookupTxReqMsg = Message<Address>;
pub type LookupTxResMsg = Message<bool>;

pub fn check_lookup_node_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
    msg.check()?;

    if msg.session.permission > Permission::Read {
        return Err(format!("invalid permission"));
    }

    if msg.method != Method::Lookup {
        return Err(format!("invalid method"));
    }

    if msg.resource != Resource::Node {
        return Err(format!("invalid resource"));
    }

    Ok(())
}

pub type GetTxReqMsg = Message<Address>;
pub type GetTxResMsg = Message<Node>;

pub fn check_get_node_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
    msg.check()?;

    if msg.session.permission > Permission::Read {
        return Err(format!("invalid permission"));
    }

    if msg.method != Method::Get {
        return Err(format!("invalid method"));
    }

    if msg.resource != Resource::Node {
        return Err(format!("invalid resource"));
    }

    Ok(())
}