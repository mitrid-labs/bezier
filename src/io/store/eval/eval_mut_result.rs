use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum StoreEvalMutResult {
    None,
    Dropped,
    DroppedRange,
    DroppedPrefix,
}

impl Default for StoreEvalMutResult {
    fn default() -> StoreEvalMutResult {
        StoreEvalMutResult::None
    }
}

impl Sizable for StoreEvalMutResult {
    fn size(&self) -> u64 {
        0u8.size()
    }
}

impl Checkable for StoreEvalMutResult {
    fn check(&self) -> Result<()> {
        Ok(())
    }
}

impl Serializable for StoreEvalMutResult {}

impl Datable for StoreEvalMutResult {}