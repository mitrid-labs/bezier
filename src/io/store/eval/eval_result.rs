use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

use io::Session;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[allow(unused_attributes)]
pub enum StoreEvalResult {
    #[repr(u8)]
    None,
    Path { path: String },
    SessionsPrefix { prefix: Vec<u8> },
    CountSessions { count: u64 },
    ListSessions { sessions: Vec<Session> },
    GetSession { session: Session },
    Size { size: u64 },
    SizeRange { size: u64 },
    SizePrefix { size: u64 },
    Dump { items: Vec<(Vec<u8>, Vec<u8>)> },
    DumpRange { items: Vec<(Vec<u8>, Vec<u8>)> },
    DumpPrefix { items: Vec<(Vec<u8>, Vec<u8>)> },
}

impl StoreEvalResult {
    pub fn new_none() -> StoreEvalResult {
        StoreEvalResult::None
    }

    pub fn new_path(path: &str) -> StoreEvalResult {
        StoreEvalResult::Path { path: path.to_owned() }
    }

    pub fn new_sessions_prefix(prefix: &[u8]) -> StoreEvalResult {
        StoreEvalResult::SessionsPrefix { prefix: prefix.to_owned() }
    }

    pub fn new_count_sessions(count: u64) -> StoreEvalResult {
        StoreEvalResult::CountSessions { count }
    }

    pub fn new_list_sessions(sessions: &Vec<Session>) -> Result<StoreEvalResult> {
        sessions.check()?;

        let result = StoreEvalResult::ListSessions {
            sessions: sessions.to_owned(),
        };

        Ok(result)
    }

    pub fn new_get_session(session: &Session) -> Result<StoreEvalResult> {
        session.check()?;

        let result = StoreEvalResult::GetSession {
            session: session.to_owned(),
        };

        Ok(result)
    }

    pub fn new_size(size: u64) -> StoreEvalResult {
        StoreEvalResult::Size { size }
    }

    pub fn new_size_range(size: u64) -> StoreEvalResult {
        StoreEvalResult::SizeRange { size }
    }

    pub fn new_size_prefix(size: u64) -> StoreEvalResult {
        StoreEvalResult::SizePrefix { size }
    }

    pub fn new_dump(items: &Vec<(Vec<u8>, Vec<u8>)>) -> StoreEvalResult {
        StoreEvalResult::Dump {
            items: items.to_owned()
        }
    }

    pub fn new_dump_range(items: &Vec<(Vec<u8>, Vec<u8>)>) -> StoreEvalResult {
        StoreEvalResult::DumpRange {
            items: items.to_owned()
        }
    }

    pub fn new_dump_prefix(items: &Vec<(Vec<u8>, Vec<u8>)>) -> StoreEvalResult {
        StoreEvalResult::DumpPrefix {
            items: items.to_owned()
        }
    }
}

impl Default for StoreEvalResult {
    fn default() -> StoreEvalResult {
        StoreEvalResult::None
    }
}

impl Sizable for StoreEvalResult {
    fn size(&self) -> u64 {
        match self {
            &StoreEvalResult::None => 0u8.size(),
            &StoreEvalResult::Path { ref path } => path.size(),
            &StoreEvalResult::SessionsPrefix { ref prefix } => prefix.size(),
            &StoreEvalResult::CountSessions { count } => count.size(),
            &StoreEvalResult::ListSessions { ref sessions } => sessions.size(),
            &StoreEvalResult::GetSession { ref session } => session.size(),
            &StoreEvalResult::Size { ref size } => size.size(),
            &StoreEvalResult::SizeRange { ref size } => size.size(),
            &StoreEvalResult::SizePrefix { ref size } => size.size(),
            &StoreEvalResult::Dump { ref items } => items.size(),
            &StoreEvalResult::DumpRange { ref items } => items.size(),
            &StoreEvalResult::DumpPrefix { ref items } => items.size(),
        }
    }
}

impl Checkable for StoreEvalResult {
    fn check(&self) -> Result<()> {
        match self {
            &StoreEvalResult::None => Ok(()),
            &StoreEvalResult::Path { ref path } => path.check(),
            &StoreEvalResult::SessionsPrefix { ref prefix } => prefix.check(),
            &StoreEvalResult::CountSessions { count } => count.check(),
            &StoreEvalResult::ListSessions { ref sessions } => sessions.check(),
            &StoreEvalResult::GetSession { ref session } => session.check(),
            &StoreEvalResult::Size { ref size } => size.check(),
            &StoreEvalResult::SizeRange { ref size } => size.check(),
            &StoreEvalResult::SizePrefix { ref size } => size.check(),
            &StoreEvalResult::Dump { ref items } => items.check(),
            &StoreEvalResult::DumpRange { ref items } => items.check(),
            &StoreEvalResult::DumpPrefix { ref items } => items.check(),
        }
    }
}

impl Serializable for StoreEvalResult {}

impl Datable for StoreEvalResult {}