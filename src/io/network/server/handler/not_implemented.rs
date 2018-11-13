use mitrid_core::base::Result;
use mitrid_core::io::Resource;

use crypto::Hasher;
use io::Store;
use io::{Message, Request, Response};
use io::check_req;

pub fn not_implemented(_store: &mut Store,
                       request: &Request)
    -> Result<Response>
{
    check_req(request)?;

    let mut hasher = Hasher{};

    let payload = "not not_implemented".as_bytes().to_vec();

    let message = Message::new()
                    .meta(&request.message.meta)?
                    .session(&request.message.session)?
                    .method(&request.message.method)?
                    .resource(&Resource::Error)?
                    .payload(&payload)?
                    .finalize(&mut hasher)?;

    Response::new(&message)
}