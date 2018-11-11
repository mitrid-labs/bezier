use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::io::{Permission, Method, Resource};

use io::Message;

pub type PingMsg = Message<()>;

pub fn check_ping_msg(msg: &PingMsg) -> Result<()> {
    msg.check()?;

    if msg.session.is_expired()? {
        return Err(format!("expired session"));
    }

    if msg.session.permission > Permission::None {
        return Err(format!("invalid permission"));
    }

    if msg.method != Method::Ping {
        return Err(format!("invalid method"));
    }

    if msg.resource != Resource::None {
        return Err(format!("invalid resource"));
    }

    Ok(())
}