use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use io::network::message::message::node::*;
use io::network::message::response::response::*;

pub struct NodeResponse;

impl NodeResponse {
    pub fn verify_count(res: &Response) -> Result<bool> {
        res.check()?;

        NodeMessage::verify_count(&res.message)
    }

    pub fn verify_list(res: &Response) -> Result<bool> {
        res.check()?;

        NodeMessage::verify_list(&res.message)
    }

    pub fn verify_lookup(res: &Response) -> Result<bool> {
        res.check()?;

        NodeMessage::verify_lookup(&res.message)
    }

    pub fn verify_get(res: &Response) -> Result<bool> {
        res.check()?;

        NodeMessage::verify_get(&res.message)
    }

    pub fn check_count(res: &Response) -> Result<()> {
        res.check()?;

        NodeMessage::check_count(&res.message)
    }

    pub fn check_list(res: &Response) -> Result<()> {
        res.check()?;

        NodeMessage::check_list(&res.message)
    }

    pub fn check_lookup(res: &Response) -> Result<()> {
        res.check()?;

        NodeMessage::check_lookup(&res.message)
    }

    pub fn check_get(res: &Response) -> Result<()> {
        res.check()?;

        NodeMessage::check_get(&res.message)
    }

}