use mitrid_core::model::Output as OutputBase;

use crypto::Digest;
use model::Amount;
use model::output::OutputPayload;

pub type Output = OutputBase<Digest, Amount, OutputPayload>;