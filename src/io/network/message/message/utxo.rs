use mitrid_core::base::Result;
use mitrid_core::io::{Method, Resource};

use io::network::message::message::message::*;

pub struct UTxOMessage;

impl UTxOMessage {
    pub fn verify_count(msg: &Message) -> Result<bool> {
        verify_read_msg(msg, &Method::Count, &Resource::Coin)
    }

    pub fn verify_list(msg: &Message) -> Result<bool> {
        verify_read_msg(msg, &Method::List, &Resource::Coin)
    }

    pub fn verify_lookup(msg: &Message) -> Result<bool> {
        verify_read_msg(msg, &Method::Lookup, &Resource::Coin)
    }

    pub fn verify_get(msg: &Message) -> Result<bool> {
        verify_read_msg(msg, &Method::Get, &Resource::Coin)
    }

    pub fn check_count(msg: &Message) -> Result<()> {
        check_read_msg(msg, &Method::Count, &Resource::Coin)
    }

    pub fn check_list(msg: &Message) -> Result<()> {
        check_read_msg(msg, &Method::List, &Resource::Coin)
    }

    pub fn check_lookup(msg: &Message) -> Result<()> {
        check_read_msg(msg, &Method::Lookup, &Resource::Coin)
    }

    pub fn check_get(msg: &Message) -> Result<()> {
        check_read_msg(msg, &Method::Get, &Resource::Coin)
    }
}