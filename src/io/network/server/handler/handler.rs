use mitrid_core::base::Result;
use mitrid_core::io::Handler as HandlerBase;

use crypto::Digest;
use io::Store;
use io::{Request, Response};
use io::network::server::handler::*;

#[derive(Clone)]
pub struct Handler;

impl HandlerBase<Store, (), (), Digest, Vec<u8>> for Handler {
    fn handle_ping(&mut self,
                   store: &mut Store,
                   request: &Request)
      -> Result<Response>
    {
        ping(store, request)
          .or_else(|e| error(store, request, &format!("{}", e)))
    }

    fn handle_session(&mut self,
                      store: &mut Store,
                      request: &Request)
        -> Result<Response>
    {
        session(store, request)
          .or_else(|e| error(store, request, &format!("{}", e)))
    }

    fn handle_count(&mut self,
                    store: &mut Store,
                    request: &Request)
        -> Result<Response>
    {
        count(store, request)
          .or_else(|e| error(store, request, &format!("{}", e)))
    }

    fn handle_list(&mut self,
                   store: &mut Store,
                   request: &Request)
        -> Result<Response>
    {
        list(store, request)
          .or_else(|e| error(store, request, &format!("{}", e)))
    }

    fn handle_lookup(&mut self,
                     store: &mut Store,
                     request: &Request)
        -> Result<Response>
    {
        lookup(store, request)
          .or_else(|e| error(store, request, &format!("{}", e)))
    }

    fn handle_get(&mut self,
                  store: &mut Store,
                  request: &Request)
        -> Result<Response>
    {
        get(store, request)
          .or_else(|e| error(store, request, &format!("{}", e)))
    }

    fn handle_create(&mut self,
                     store: &mut Store,
                     request: &Request)
        -> Result<Response>
    {
        create(store, request)
          .or_else(|e| error(store, request, &format!("{}", e)))
    }

    fn handle_update(&mut self,
                     store: &mut Store,
                     request: &Request)
        -> Result<Response>
    {
        error(store, request, "invalid path")
    }

    fn handle_upsert(&mut self,
                     store: &mut Store,
                     request: &Request)
        -> Result<Response>
    {
        error(store, request, "invalid path")
    }

    fn handle_delete(&mut self,
                     store: &mut Store,
                     request: &Request)
        -> Result<Response>
    {
        error(store, request, "invalid path")
    }
}