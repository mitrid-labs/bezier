use mitrid_core::base::Result;
use mitrid_core::base::{Sizable, ConstantSize};
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;
use mitrid_core::crypto::Sign;

use crypto::Digest;
use crypto::{Signer, SecretKey, PublicKey, Signature};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct OutputPayload {
    pub digest: Digest,
    pub public_key: PublicKey,
    pub signature: Signature,
}

impl OutputPayload {
    pub fn new(digest: &Digest, public_key: &PublicKey, secret_key: &SecretKey) -> Result<OutputPayload> {
        digest.check()?;
        digest.check_size()?;
        public_key.check()?;
        public_key.check_size()?;
        secret_key.check()?;
        secret_key.check_size()?;

        let mut signer = Signer{};
        let msg = digest.as_slice();
        
        let signature = signer.sign(msg, secret_key)?;
        signer.check(msg, public_key, &signature)?;

        let payload = OutputPayload {
            digest: digest.to_owned(),
            public_key: public_key.to_owned(),
            signature: signature,
        };

        Ok(payload)
    }

    pub fn verify_signature(&self) -> Result<bool> {
        self.digest.check()?;
        self.digest.check_size()?;
        self.public_key.check()?;
        self.public_key.check_size()?;
        self.signature.check()?;
        self.signature.check_size()?;
        
        let mut signer = Signer{};
        let msg = self.digest.as_slice();

        signer.verify(msg, &self.public_key, &self.signature)
    }

    pub fn check_signature(&self) -> Result<()> {
        self.digest.check()?;
        self.digest.check_size()?;
        self.public_key.check()?;
        self.public_key.check_size()?;
        self.signature.check()?;
        self.signature.check_size()?;

        let mut signer = Signer{};
        let msg = self.digest.as_slice();

        signer.check(msg, &self.public_key, &self.signature)
    }
}

impl Sizable for OutputPayload {
    fn size(&self) -> u64 {
        self.digest.size() +
        self.public_key.size() +
        self.signature.size()
    }
}

impl Checkable for OutputPayload {
    fn check(&self) -> Result<()> {
        self.digest.check()?;
        self.digest.check_size()?;
        self.public_key.check()?;
        self.public_key.check_size()?;
        self.signature.check()?;
        self.signature.check_size()?;

        let mut signer = Signer{};
        let msg = self.digest.as_slice();

        signer.check(msg, &self.public_key, &self.signature)
    }
}

impl Serializable for OutputPayload {}

impl Datable for OutputPayload {}