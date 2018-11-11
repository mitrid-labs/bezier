use mitrid_core::base::Datable;

use io::Store;
use io::{Request, ErrorResponse};

pub fn not_implemented<P: Datable>(&mut self,
                                   _store: &mut Store,
                                   request: &Request<P>)
    -> Result<ErrorResponse>
{
    let error_message = ErrorMsg::new()
                                .meta(&request.meta)
                                .unwrap()
                                .session(&request.session)
                                .
}