use mitrid_core::model::Input as InputBase;

use crypto::Digest;
use model::Amount;
use model::input::InputPayload;

pub type Input = InputBase<Digest, Amount, Option<InputPayload>>;