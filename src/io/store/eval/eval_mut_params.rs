use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[allow(unused_attributes)]
pub enum StoreEvalMutParams {
    #[repr(u8)]
    None,
    #[repr(u8)]
    Drop,
    DropRange { prefix: Vec<u8>, from: Option<Vec<u8>>, to: Option<Vec<u8>> },
    DropPrefix { prefix: Vec<u8> },
}

impl StoreEvalMutParams {
    pub fn new_none() -> StoreEvalMutParams {
        StoreEvalMutParams::None
    }

    pub fn new_drop() -> StoreEvalMutParams {
        StoreEvalMutParams::Drop
    }

    pub fn new_drop_range(prefix: &[u8], from: Option<Vec<u8>>, to: Option<Vec<u8>>)
        -> Result<StoreEvalMutParams>
    {

        if let Some(ref from) = from {
            if let Some(ref to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        let params = StoreEvalMutParams::DropRange {
            prefix: prefix.to_owned(),
            from: from,
            to: to,
        };

        Ok(params)
    }

    pub fn new_drop_prefix(prefix: &[u8]) -> StoreEvalMutParams {
        StoreEvalMutParams::DropPrefix { prefix: prefix.to_owned() }
    }
}

impl Default for StoreEvalMutParams {
    fn default() -> StoreEvalMutParams {
        StoreEvalMutParams::None
    }
}

impl Sizable for StoreEvalMutParams {
    fn size(&self) -> u64 {
        match self {
            &StoreEvalMutParams::None => 0u8.size(),
            &StoreEvalMutParams::Drop => 0u8.size(),
            &StoreEvalMutParams::DropRange { ref prefix, ref from, ref to } => {
                prefix.size() + from.size() + to.size()
            },
            &StoreEvalMutParams::DropPrefix { ref prefix } => prefix.size(),
        }
    }
}

impl Checkable for StoreEvalMutParams {
    fn check(&self) -> Result<()> {
        match self {
            &StoreEvalMutParams::None => Ok(()),
            &StoreEvalMutParams::Drop => Ok(()),
            &StoreEvalMutParams::DropRange { ref prefix, ref from, ref to } => {
                prefix.check()?;
                from.check()?;
                to.check()
            },
            &StoreEvalMutParams::DropPrefix { ref prefix } => prefix.check(),
        }
    }
}

impl Serializable for StoreEvalMutParams {}

impl Datable for StoreEvalMutParams {}