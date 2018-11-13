use mitrid_core::base::Result;

use io::network::message::message::block::*;
use io::network::message::request::request::*;

pub struct BlockRequest;

impl BlockRequest {
    pub fn verify_lookup(req: &Request) -> Result<bool> {
        check_req(req)?;

        BlockMessage::verify_lookup(&req.message)
    }

    pub fn verify_get(req: &Request) -> Result<bool> {
        check_req(req)?;

        BlockMessage::verify_get(&req.message)
    }

    pub fn verify_create(req: &Request) -> Result<bool> {
        check_req(req)?;

        BlockMessage::verify_create(&req.message)
    }

    pub fn check_lookup(req: &Request) -> Result<()> {
        check_req(req)?;

        BlockMessage::check_lookup(&req.message)
    }

    pub fn check_get(req: &Request) -> Result<()> {
        check_req(req)?;

        BlockMessage::check_get(&req.message)
    }

    pub fn check_create(req: &Request) -> Result<()> {
        check_req(req)?;

        BlockMessage::check_create(&req.message)
    }
}