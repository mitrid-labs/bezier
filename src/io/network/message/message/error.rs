use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::io::Resource;

use io::Message;

pub type ErrorMsg = Message<String>;

pub fn check_error_msg(msg: &ErrorMsg) -> Result<()> {
    msg.check()?;

    if msg.resource != Resource::Error {
        return Err(format!("invalid resource"));
    }

    Ok(())
}