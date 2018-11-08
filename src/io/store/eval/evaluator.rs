use mitrid_core::base::Result;
use mitrid_core::base::{Eval, EvalMut};

use io::Store;
use io::store::eval::*;

#[derive(Clone)]
pub struct StoreEvaluator {}

impl Eval<Store, StoreEvalParams, StoreEvalResult> for StoreEvaluator {
    fn eval(&self, store: &Store, params: &StoreEvalParams) -> Result<StoreEvalResult> {
        match params {
            &StoreEvalParams::None => Ok(StoreEvalResult::None),
            &StoreEvalParams::Path => {
                let path = store.path();
                let result = StoreEvalResult::Path { path };
                Ok(result)
            },
            &StoreEvalParams::SessionsPrefix => {
                let prefix = Store::sessions_prefix();
                let result = StoreEvalResult::SessionsPrefix { prefix };
                Ok(result)
            },
            &StoreEvalParams::CountSessions => {
                let count = store.count_sessions()?;
                let result = StoreEvalResult::CountSessions { count };
                Ok(result)
            },
            &StoreEvalParams::ListSessions => {
                let sessions = store.list_sessions()?;
                let result = StoreEvalResult::ListSessions { sessions };
                Ok(result)
            },
            &StoreEvalParams::GetSession { id } => {
                let session = store.get_session(id)?;
                let result = StoreEvalResult::GetSession { session };
                Ok(result)
            },
            &StoreEvalParams::Size => {
                let size = store.size()?;
                let result = StoreEvalResult::Size { size };
                Ok(result)
            },
            &StoreEvalParams::SizeRange { ref prefix, ref from, ref to } => {
                let from = from.clone().map(|k| {
                    let mut from = Vec::new();

                    from.extend_from_slice(prefix);
                    from.extend_from_slice(&k);
                    
                    from
                });

                let to = to.clone().map(|k| {
                    let mut to = Vec::new();
                    
                    to.extend_from_slice(&prefix);
                    to.extend_from_slice(&k);
                    
                    to
                });

                let size = store.size_range(from, to)?;
                let result = StoreEvalResult::SizeRange { size };
                Ok(result)
            },
            &StoreEvalParams::SizePrefix { ref prefix } => {
                let size = store.size_prefix(prefix)?;
                let result = StoreEvalResult::SizePrefix { size };
                Ok(result)
            },
            &StoreEvalParams::Dump => {
                let items = store.dump()?;
                let result = StoreEvalResult::Dump { items };
                Ok(result)
            },
            &StoreEvalParams::DumpRange { ref prefix, ref from, ref to } => {
                let from = from.clone().map(|k| {
                    let mut from = Vec::new();

                    from.extend_from_slice(prefix);
                    from.extend_from_slice(&k);
                    
                    from
                });

                let to = to.clone().map(|k| {
                    let mut to = Vec::new();
                    
                    to.extend_from_slice(&prefix);
                    to.extend_from_slice(&k);
                    
                    to
                });

                let items = store.dump_range(from, to)?;
                let result = StoreEvalResult::DumpRange { items };
                Ok(result)
            },
            &StoreEvalParams::DumpPrefix { ref prefix } => {
                let items = store.dump_prefix(prefix)?;
                let result = StoreEvalResult::DumpPrefix { items };
                Ok(result)
            },
        }
    }
}

impl EvalMut<Store, StoreEvalMutParams, StoreEvalMutResult> for StoreEvaluator {
    fn eval_mut(&mut self, store: &mut Store, params: &StoreEvalMutParams) -> Result<StoreEvalMutResult> {
        match params {
            &StoreEvalMutParams::None => Ok(StoreEvalMutResult::None),
            &StoreEvalMutParams::Drop => {
                store.drop()?;
                
                Ok(StoreEvalMutResult::Dropped)
            },
            &StoreEvalMutParams::DropRange { ref prefix, ref from, ref to } => {
                let from = from.clone().map(|k| {
                    let mut from = Vec::new();

                    from.extend_from_slice(prefix);
                    from.extend_from_slice(&k);
                    
                    from
                });

                let to = to.clone().map(|k| {
                    let mut to = Vec::new();
                    
                    to.extend_from_slice(&prefix);
                    to.extend_from_slice(&k);
                    
                    to
                });

                store.drop_range(from, to)?;
                
                Ok(StoreEvalMutResult::DroppedRange)
            },
            &StoreEvalMutParams::DropPrefix { ref prefix } => {
                store.drop_prefix(prefix)?;

                Ok(StoreEvalMutResult::DroppedPrefix)
            },
        }
    }
}