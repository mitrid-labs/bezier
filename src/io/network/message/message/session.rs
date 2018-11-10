use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Datable;
use mitrid_core::io::{Permission, Method, Resource};

use io::Session;
use io::Message;

pub type SessionReqMsg = Message<Permission>;
pub type SessionResMsg = Message<Session>;

pub fn check_session_msg<P: Datable>(msg: &Message<P>) -> Result<()> {
    msg.check()?;

    if msg.session.is_expired()? {
        return Err(format!("expired session"));
    }

    if msg.method != Method::Session {
        return Err(format!("invalid method"));
    }

    if msg.resource != Resource::Session {
        return Err(format!("invalid resource"));
    }

    Ok(())
}