use mitrid_core::base::Result;
use mitrid_core::base::Checkable;

use model::Transaction;
use io::network::message::message::transaction::*;
use io::network::message::response::response::*;

pub struct TransactionResponse;

impl TransactionResponse {
    pub fn verify_lookup(res: &Response) -> Result<bool> {
        res.check()?;

        TransactionMessage::verify_lookup(&res.message)
    }

    pub fn verify_get(res: &Response) -> Result<bool> {
        res.check()?;

        TransactionMessage::verify_get(&res.message)
    }

    pub fn verify_create(res: &Response) -> Result<bool> {
        res.check()?;

        TransactionMessage::verify_create(&res.message)
    }

    pub fn check_lookup(res: &Response) -> Result<()> {
        res.check()?;

        TransactionMessage::check_lookup(&res.message)
    }

    pub fn check_get(res: &Response) -> Result<()> {
        res.check()?;

        TransactionMessage::check_get(&res.message)
    }

    pub fn check_create(res: &Response) -> Result<()> {
        res.check()?;

        TransactionMessage::check_create(&res.message)
    }

    pub fn parse_lookup(res: &Response) -> Result<bool> {
        parse_res(res)
    }

    pub fn parse_get(res: &Response) -> Result<Transaction> {
        parse_res(res)
    }

    pub fn parse_create(res: &Response) -> Result<()> {
        parse_res(res)
    }
}