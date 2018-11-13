use mitrid_core::base::Result;
use mitrid_core::io::{Method, Resource};

use io::network::message::message::message::*;

pub struct TransactionMessage;

impl TransactionMessage {
    pub fn verify_lookup(msg: &Message) -> Result<bool> {
        verify_read_msg(msg, &Method::Lookup, &Resource::Transaction)
    }

    pub fn verify_get(msg: &Message) -> Result<bool> {
        verify_read_msg(msg, &Method::Get, &Resource::Transaction)
    }

    pub fn verify_create(msg: &Message) -> Result<bool> {
        verify_write_msg(msg, &Method::Create, &Resource::Transaction)
    }

    pub fn check_lookup(msg: &Message) -> Result<()> {
        check_read_msg(msg, &Method::Lookup, &Resource::Transaction)
    }

    pub fn check_get(msg: &Message) -> Result<()> {
        check_read_msg(msg, &Method::Get, &Resource::Transaction)
    }

    pub fn check_create(msg: &Message) -> Result<()> {
        check_write_msg(msg, &Method::Create, &Resource::Transaction)
    }
}