use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[allow(unused_attributes)]
pub enum StoreEvalParams {
    #[repr(u8)]
    None,
    #[repr(u8)]
    Path,
    #[repr(u8)]
    SessionsPrefix,
    #[repr(u8)]
    CountSessions,
    #[repr(u8)]
    ListSessions,
    GetSession { id: u64 },
    #[repr(u8)]
    Size,
    SizeRange { prefix: Vec<u8>, from: Option<Vec<u8>>, to: Option<Vec<u8>> },
    SizePrefix { prefix: Vec<u8> },
    #[repr(u8)]
    Dump,
    DumpRange { prefix: Vec<u8>, from: Option<Vec<u8>>, to: Option<Vec<u8>> },
    DumpPrefix { prefix: Vec<u8> },
}

impl StoreEvalParams {
    pub fn new_none() -> StoreEvalParams {
        StoreEvalParams::None
    }

    pub fn path() -> StoreEvalParams {
        StoreEvalParams::Path
    }

    pub fn new_sessions_prefix() -> StoreEvalParams {
        StoreEvalParams::SessionsPrefix
    }

    pub fn new_count_sessions() -> StoreEvalParams {
        StoreEvalParams::CountSessions
    }

    pub fn new_list_sessions() -> StoreEvalParams {
        StoreEvalParams::ListSessions
    }

    pub fn new_get_session(id: u64) -> StoreEvalParams {
        StoreEvalParams::GetSession { id }
    }

    pub fn new_size() -> StoreEvalParams {
        StoreEvalParams::Size
    }

    pub fn new_size_range(prefix: &[u8], from: Option<Vec<u8>>, to: Option<Vec<u8>>)
        -> Result<StoreEvalParams>
    {

        if let Some(ref from) = from {
            if let Some(ref to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        let params = StoreEvalParams::SizeRange {
            prefix: prefix.to_owned(),
            from: from,
            to: to,
        };

        Ok(params)
    }

    pub fn new_size_prefix(prefix: &[u8]) -> StoreEvalParams {
        StoreEvalParams::SizePrefix { prefix: prefix.to_owned() }
    }

    pub fn new_dump() -> StoreEvalParams {
        StoreEvalParams::Dump
    }

    pub fn new_dump_range(prefix: &[u8], from: Option<Vec<u8>>, to: Option<Vec<u8>>)
        -> Result<StoreEvalParams>
    {

        if let Some(ref from) = from {
            if let Some(ref to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        let params = StoreEvalParams::DumpRange {
            prefix: prefix.to_owned(),
            from: from,
            to: to,
        };

        Ok(params)
    }

    pub fn new_dump_prefix(prefix: &[u8]) -> StoreEvalParams {
        StoreEvalParams::DumpPrefix { prefix: prefix.to_owned() }
    }
}

impl Default for StoreEvalParams {
    fn default() -> StoreEvalParams {
        StoreEvalParams::None
    }
}

impl Sizable for StoreEvalParams {
    fn size(&self) -> u64 {
        match self {
            &StoreEvalParams::None => 0u8.size(),
            &StoreEvalParams::Path => 0u8.size(),
            &StoreEvalParams::SessionsPrefix => 0u8.size(),
            &StoreEvalParams::CountSessions => 0u8.size(),
            &StoreEvalParams::ListSessions => 0u8.size(),
            &StoreEvalParams::GetSession { id } => id.size(),
            &StoreEvalParams::Size => 0u8.size(),
            &StoreEvalParams::SizeRange { ref prefix, ref from, ref to } => {
                prefix.size() + from.size() + to.size()
            },
            &StoreEvalParams::SizePrefix { ref prefix } => prefix.size(),
            &StoreEvalParams::Dump => 0u8.size(),
            &StoreEvalParams::DumpRange { ref prefix, ref from, ref to } => {
                prefix.size() + from.size() + to.size()
            },
            &StoreEvalParams::DumpPrefix { ref prefix } => prefix.size(),
        }
    }
}

impl Checkable for StoreEvalParams {
    fn check(&self) -> Result<()> {
        match self {
            &StoreEvalParams::None => Ok(()),
            &StoreEvalParams::Path => Ok(()),
            &StoreEvalParams::SessionsPrefix => Ok(()),
            &StoreEvalParams::CountSessions => Ok(()),
            &StoreEvalParams::ListSessions => Ok(()),
            &StoreEvalParams::GetSession { id } => id.check(),
            &StoreEvalParams::Size => Ok(()),
            &StoreEvalParams::SizeRange { ref prefix, ref from, ref to } => {
                prefix.check()?;
                from.check()?;
                to.check()
            },
            &StoreEvalParams::SizePrefix { ref prefix } => prefix.check(),
            &StoreEvalParams::Dump => Ok(()),
            &StoreEvalParams::DumpRange { ref prefix, ref from, ref to } => {
                prefix.check()?;
                from.check()?;
                to.check()
            },
            &StoreEvalParams::DumpPrefix { ref prefix } => prefix.check(),
        }
    }
}

impl Serializable for StoreEvalParams {}

impl Datable for StoreEvalParams {}