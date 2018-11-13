use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::io::Storable;

use crypto::Hasher;
use io::Store;
use io::network::{Message, Request, Response};
use io::network::message::request::transaction::*;
use io::network::message::request::block::*;

pub fn create_handler(store: &mut Store,
                      request: &Request)
    -> Result<Response>
{
    request.check()?;

    if TransactionRequest::verify_create(request)? {
        let transaction = TransactionRequest::parse_create(request)?;
        transaction.store_create(store)?;
        // TODO: store input coins per transaction
    } else if BlockRequest::verify_create(request)? {
        let block = BlockRequest::parse_create(request)?;

        // TODO: check if txs exist
        //       check if tx input coins are in store
        //       whatever: make a list
        block.store_create(store)?;
    } else {
        return Err(format!("invalid request"));
    };

    let mut hasher = Hasher{};

    let message = Message::new()
                        .meta(&request.message.meta)?
                        .session(&request.message.session)?
                        .method(&request.message.method)?
                        .resource(&request.message.resource)?
                        .payload(&().to_bytes()?)?
                        .finalize(&mut hasher)?;

    Response::new(&message)
}