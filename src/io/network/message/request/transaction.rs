use mitrid_core::base::Result;
use mitrid_core::base::ConstantSize;
use mitrid_core::base::Checkable;

use crypto::Digest;
use model::Transaction;
use io::network::message::message::transaction::*;
use io::network::message::request::request::*;

pub struct TransactionRequest;

impl TransactionRequest {
    pub fn verify_lookup(req: &Request) -> Result<bool> {
        check_req(req)?;

        TransactionMessage::verify_lookup(&req.message)
    }

    pub fn verify_get(req: &Request) -> Result<bool> {
        check_req(req)?;

        TransactionMessage::verify_get(&req.message)
    }

    pub fn verify_create(req: &Request) -> Result<bool> {
        check_req(req)?;

        TransactionMessage::verify_create(&req.message)
    }

    pub fn check_lookup(req: &Request) -> Result<()> {
        check_req(req)?;

        TransactionMessage::check_lookup(&req.message)
    }

    pub fn check_get(req: &Request) -> Result<()> {
        check_req(req)?;

        TransactionMessage::check_get(&req.message)
    }

    pub fn check_create(req: &Request) -> Result<()> {
        check_req(req)?;

        TransactionMessage::check_create(&req.message)
    }

    pub fn parse_lookup(req: &Request) -> Result<Digest> {
        let digest: Digest = parse_req(req)?;

        digest.check()?;
        digest.check_size()?;

        Ok(digest)
    }

    pub fn parse_get(req: &Request) -> Result<Digest> {
        let digest: Digest = parse_req(req)?;

        digest.check()?;
        digest.check_size()?;

        Ok(digest)
    }

    pub fn parse_create(req: &Request) -> Result<Transaction> {
        let transaction: Transaction = parse_req(req)?;

        transaction.check()?;

        Ok(transaction)
    }
}