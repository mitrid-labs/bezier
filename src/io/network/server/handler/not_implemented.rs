use mitrid_core::base::Result;
use mitrid_core::base::Datable;

use crypto::Hasher;
use io::Store;
use io::{Request, ErrorMsg, ErrorResponse};

pub fn not_implemented<P: Datable>(_store: &mut Store,
                                   request: &Request<P>)
    -> Result<ErrorResponse>
{
    let mut hasher = Hasher{};

    let payload = String::from("not not_implemented");

    let error_message = ErrorMsg::new()
                                .meta(&request.message.meta)?
                                .session(&request.message.session)?
                                .method(&request.message.method)?
                                .resource(&request.message.resource)?
                                .payload(&payload)?
                                .finalize(&mut hasher)?;

    ErrorResponse::new(&error_message)
}