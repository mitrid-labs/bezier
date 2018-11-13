use mitrid_core::base::Result;
use mitrid_core::base::VariableSize;
use mitrid_core::base::Checkable;

use io::Address;
use io::network::message::message::node::*;
use io::network::message::request::request::*;

pub struct NodeRequest;

impl NodeRequest {
    pub fn verify_count(req: &Request) -> Result<bool> {
        check_req(req)?;

        NodeMessage::verify_count(&req.message)
    }

    pub fn verify_list(req: &Request) -> Result<bool> {
        check_req(req)?;

        NodeMessage::verify_list(&req.message)
    }

    pub fn verify_lookup(req: &Request) -> Result<bool> {
        check_req(req)?;

        NodeMessage::verify_lookup(&req.message)
    }

    pub fn verify_get(req: &Request) -> Result<bool> {
        check_req(req)?;

        NodeMessage::verify_get(&req.message)
    }

    pub fn check_count(req: &Request) -> Result<()> {
        check_req(req)?;

        NodeMessage::check_count(&req.message)
    }

    pub fn check_list(req: &Request) -> Result<()> {
        check_req(req)?;

        NodeMessage::check_list(&req.message)
    }

    pub fn check_lookup(req: &Request) -> Result<()> {
        check_req(req)?;

        NodeMessage::check_lookup(&req.message)
    }

    pub fn check_get(req: &Request) -> Result<()> {
        check_req(req)?;

        NodeMessage::check_get(&req.message)
    }

    pub fn parse_count(req: &Request) -> Result<(Option<Address>, Option<Address>)> {
        let (from, to): (Option<Address>, Option<Address>) = parse_req(req)?;

        from.check()?;

        to.check()?;

        if let Some(ref from) = from {
            from.check_size()?;
            
            if let Some(ref to) = to {
                to.check_size()?;

                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        Ok((from, to))
    }

    pub fn parse_list(req: &Request) -> Result<(Option<Address>, Option<Address>, Option<u64>)> {
        let (from, to, count): (Option<Address>, Option<Address>, Option<u64>) = parse_req(req)?;

        from.check()?;

        to.check()?;

        if let Some(ref from) = from {
            from.check_size()?;
            
            if let Some(ref to) = to {
                to.check_size()?;

                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        if let Some(count) = count {
            if count == 0 {
                return Err(String::from("invalid count"));
            }
        }

        Ok((from, to, count))
    }

    pub fn parse_lookup(req: &Request) -> Result<Address> {
        let address: Address = parse_req(req)?;

        address.check()?;
        address.check_size()?;

        Ok(address)
    }

    pub fn parse_get(req: &Request) -> Result<Address> {
        let address: Address = parse_req(req)?;

        address.check()?;
        address.check_size()?;

        Ok(address)
    }
}