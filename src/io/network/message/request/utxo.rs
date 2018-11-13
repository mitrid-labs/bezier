use mitrid_core::base::Result;

use crypto::Digest;
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

    pub fn parse_count(req: &Request) -> Result<(Option<Digest>, Option<Digest>)> {
        let (from, to): (Option<Digest>, Option<Digest>) = parse_req(req)?;

        if let Some(ref from) = from {
            if let Some(ref to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        Ok((from, to))
    }

    pub fn parse_list(req: &Request) -> Result<(Option<Digest>, Option<Digest>, Option<u64>, u64)> {
        let (from, to, count, skip): (Option<Digest>, Option<Digest>, Option<u64>, u64) = parse_req(req)?;

        if let Some(ref from) = from {
            if let Some(ref to) = to {
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

        Ok((from, to, count, skip))
    }

    pub fn parse_lookup(req: &Request) -> Result<Digest> {
        parse_req(req)
    }

    pub fn parse_get(req: &Request) -> Result<Digest> {
        parse_req(req)
    }
}