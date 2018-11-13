use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use io::network::message::message::block::*;
use io::network::message::response::response::*;

pub struct BlockResponse;

impl BlockResponse {
    pub fn verify_lookup(res: &Response) -> Result<bool> {
        res.check()?;

        BlockMessage::verify_lookup(&res.message)
    }

    pub fn verify_get(res: &Response) -> Result<bool> {
        res.check()?;

        BlockMessage::verify_get(&res.message)
    }

    pub fn verify_create(res: &Response) -> Result<bool> {
        res.check()?;

        BlockMessage::verify_create(&res.message)
    }

    pub fn check_lookup(res: &Response) -> Result<()> {
        res.check()?;

        BlockMessage::check_lookup(&res.message)
    }

    pub fn check_get(res: &Response) -> Result<()> {
        res.check()?;

        BlockMessage::check_get(&res.message)
    }

    pub fn check_create(res: &Response) -> Result<()> {
        res.check()?;

        BlockMessage::check_create(&res.message)
    }
}