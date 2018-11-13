use mitrid_core::base::Result;

use crypto::Hasher;
use io::Store;
use io::{Message, Response};
use io::{Request, check_req};

pub fn not_implemented(_store: &mut Store,
                       request: &Request)
    -> Result<Response>
{
    check_req(request)?;

    let mut hasher = Hasher{};

    let payload = "not not_implemented".as_bytes().to_vec();

    let error_message = Message::new()
                            .meta(&request.message.meta)?
                            .session(&request.message.session)?
                            .method(&request.message.method)?
                            .resource(&request.message.resource)?
                            .payload(&payload)?
                            .finalize(&mut hasher)?;

    Response::new(&error_message)
}