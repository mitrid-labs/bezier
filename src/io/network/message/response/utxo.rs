use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use io::network::message::message::utxo::*;
use io::network::message::response::response::*;


pub struct UTxOResponse;

impl UTxOResponse {
    pub fn verify_count(res: &Response) -> Result<bool> {
        res.check()?;

        UTxOMessage::verify_count(&res.message)
    }

    pub fn verify_list(res: &Response) -> Result<bool> {
        res.check()?;

        UTxOMessage::verify_list(&res.message)
    }

    pub fn verify_lookup(res: &Response) -> Result<bool> {
        res.check()?;

        UTxOMessage::verify_lookup(&res.message)
    }

    pub fn verify_get(res: &Response) -> Result<bool> {
        res.check()?;

        UTxOMessage::verify_get(&res.message)
    }

    pub fn check_count(res: &Response) -> Result<()> {
        res.check()?;

        UTxOMessage::check_count(&res.message)
    }

    pub fn check_list(res: &Response) -> Result<()> {
        res.check()?;

        UTxOMessage::check_list(&res.message)
    }

    pub fn check_lookup(res: &Response) -> Result<()> {
        res.check()?;

        UTxOMessage::check_lookup(&res.message)
    }

    pub fn check_get(res: &Response) -> Result<()> {
        res.check()?;

        UTxOMessage::check_get(&res.message)
    }
}