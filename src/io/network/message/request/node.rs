use mitrid_core::base::Result;

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

}