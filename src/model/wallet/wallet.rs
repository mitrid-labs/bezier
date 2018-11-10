use mitrid_core::model::Wallet as WalletBase;

use crypto::Digest;
use model::Amount;
use model::wallet::WalletPayload;

pub type Wallet = WalletBase<Digest, Amount, WalletPayload>;