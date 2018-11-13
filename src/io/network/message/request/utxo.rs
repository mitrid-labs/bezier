use mitrid_core::base::Result;

use io::network::message::message::utxo::*;
use io::network::message::request::request::*;


pub struct UTxORequest;

impl UTxORequest {
    pub fn verify_count(req: &Request) -> Result<bool> {
        check_req(req)?;

        UTxOMessage::verify_count(&req.message)
    }

    pub fn verify_list(req: &Request) -> Result<bool> {
        check_req(req)?;

        UTxOMessage::verify_list(&req.message)
    }

    pub fn verify_lookup(req: &Request) -> Result<bool> {
        check_req(req)?;

        UTxOMessage::verify_lookup(&req.message)
    }

    pub fn verify_get(req: &Request) -> Result<bool> {
        check_req(req)?;

        UTxOMessage::verify_get(&req.message)
    }

    pub fn check_count(req: &Request) -> Result<()> {
        check_req(req)?;

        UTxOMessage::check_count(&req.message)
    }

    pub fn check_list(req: &Request) -> Result<()> {
        check_req(req)?;

        UTxOMessage::check_list(&req.message)
    }

    pub fn check_lookup(req: &Request) -> Result<()> {
        check_req(req)?;

        UTxOMessage::check_lookup(&req.message)
    }

    pub fn check_get(req: &Request) -> Result<()> {
        check_req(req)?;

        UTxOMessage::check_get(&req.message)
    }
}