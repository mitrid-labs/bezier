use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Datable;
use mitrid_core::io::{Permission, Method, Resource};
use mitrid_core::io::Message as MessageBase;

use crypto::Digest;

pub type Message = MessageBase<(), Digest, Vec<u8>>;

pub fn verify_msg(msg: &Message,
                  method: &Method,
                  resource: &Resource)
    -> Result<bool>
{
    msg.check()?;

    if &msg.method != method {
        return Ok(false);
    }

    if &msg.resource != resource {
        return Ok(false);
    }

    Ok(true)
}

pub fn verify_none_msg(msg: &Message,
                       method: &Method,
                       resource: &Resource)
    -> Result<bool>
{
    if !verify_msg(msg, method, resource)? {
        return Ok(false);
    }

    if msg.session.permission != Permission::None {
        return Ok(false);
    }

    Ok(true)
}

pub fn verify_read_msg(msg: &Message,
                       method: &Method,
                       resource: &Resource)
    -> Result<bool>
{
    if !verify_msg(msg, method, resource)? {
        return Ok(false);
    }

    if msg.session.permission > Permission::Read {
        return Ok(false);
    }

    Ok(true)
}

pub fn verify_write_msg(msg: &Message,
                        method: &Method,
                        resource: &Resource)
    -> Result<bool>
{
    if !verify_msg(msg, method, resource)? {
        return Ok(false);
    }

    if msg.session.permission < Permission::Write {
        return Ok(false);
    }

    Ok(true)
}

pub fn check_msg(msg: &Message,
                 method: &Method,
                 resource: &Resource)
    -> Result<()>
{
    msg.check()?;

    if &msg.method != method {
        return Err(format!("invalid method"));
    }

    if &msg.resource != resource {
        return Err(format!("invalid resource"));
    }

    Ok(())
}

pub fn check_none_msg(msg: &Message,
                      method: &Method,
                      resource: &Resource)
    -> Result<()>
{
    check_msg(msg, method, resource)?;

    if msg.session.permission != Permission::None {
        return Err(format!("invalid permission"));
    }

    Ok(())
}

pub fn check_read_msg(msg: &Message,
                      method: &Method,
                      resource: &Resource)
    -> Result<()>
{
    check_msg(msg, method, resource)?;

    if msg.session.permission > Permission::Read {
        return Err(format!("invalid permission"));
    }

    Ok(())
}

pub fn check_write_msg(msg: &Message,
                       method: &Method,
                       resource: &Resource)
    -> Result<()>
{
    check_msg(msg, method, resource)?;

    if msg.session.permission < Permission::Write {
        return Err(format!("invalid permission"));
    }

    Ok(())
}

pub fn parse_msg<P: Datable + Serializable>(msg: &Message) -> Result<P> {
    msg.check()?;

    P::from_bytes(&msg.payload)
}