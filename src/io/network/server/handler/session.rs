use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::io::Store as StoreBase;

use crypto::Hasher;
use io::Store;
use io::{Message, Request, SessionRequest, Response};

pub fn session(store: &mut Store,
               request: &Request)
    -> Result<Response>
{
    SessionRequest::check(request)?;

    let permission = SessionRequest::parse(request)?;

    permission.check()?;

    let session = store.session(&permission)?;

    let payload = session.to_bytes()?;

    let mut hasher = Hasher{};

    let message = Message::new()
                        .meta(&request.message.meta)?
                        .session(&request.message.session)?
                        .method(&request.message.method)?
                        .resource(&request.message.resource)?
                        .payload(&payload)?
                        .finalize(&mut hasher)?;

    Response::new(&message)
}