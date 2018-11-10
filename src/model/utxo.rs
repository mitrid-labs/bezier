use mitrid_core::model::Coin as CoinBase;

use crypto::Digest;
use model::Amount;

pub type UTxO = CoinBase<Digest, Amount>;