use sodiumoxide::init;

use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::crypto::Hash;
use mitrid_core::crypto::Prove;

use std::mem;
use std::u32;

use crypto::Hasher;

// Already Datable in mitrid_core
pub type Proof = Option<u32>;

// NB: loosely hashcash
pub struct HashCash {
    pub bits: u32,
}

impl HashCash {
    pub fn new(bits: u32) -> HashCash {
        HashCash {
            bits: bits
        }
    }

    fn _prove(&self, msg: &[u8]) -> Result<Proof> {
        init().unwrap();

        let mut hasher = Hasher{};

        let base_digest = hasher.digest(msg)?;
        let mut nonce: u32 = 0;
        let mut found = false;

        let bits = self.bits;

        while !found {
            let nonce_arr: [u8; 4] = unsafe { mem::transmute(nonce) };
            let mut msg = Vec::new();
            msg.extend_from_slice(&nonce_arr[..]);
            msg.extend_from_slice(base_digest.as_slice());

            let digest = hasher.digest(&msg)?;
            let _digest = digest.as_slice();
            let mut __digest = [0u8; 4];
            for i in 0..4 {
                __digest[i] = _digest[i];
            }
            
            let leading: u32 = unsafe { mem::transmute(__digest) };
            
            if leading.leading_zeros() >= bits {
                found = true;
            } else {
                if nonce == u32::MAX {
                    break;
                }

                nonce += 1;
            }
        }

        if !found {
            Ok(None)
        } else {
            Ok(Some(nonce))
        }

    }

    fn _verify(&self, msg: &[u8], proof: &Proof) -> Result<bool> {
        init().unwrap();

        proof.check()?;

        let bits = self.bits;

        let mut hasher = Hasher{};

        if let Some(nonce) = proof {
            let nonce_arr: [u8; 4] = unsafe { mem::transmute(*nonce) };

            let base_digest = hasher.digest(msg)?;
            
            let mut msg = Vec::new();
            msg.extend_from_slice(&nonce_arr[..]);
            msg.extend_from_slice(base_digest.as_slice());

            let digest = Hasher::digest(&msg)?;
            let _digest = digest.as_slice();
            let mut __digest = [0u8; 4];
            for i in 0..4 {
                __digest[i] = _digest[i];
            }

            let leading: u32 = unsafe { mem::transmute(__digest) };

            if leading.leading_zeros() >= bits {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            return Ok(false);
        }
    }

    fn _check(&self, msg: &[u8], proof: &Proof) -> Result<()> {
        init().unwrap();

        proof.check()?;

        if !self._verify(msg, proof)? {
            return Err(format!("invalid proof"));
        }

        Ok(())
    }
}

impl Prove<Proof> for HashCash {
    fn prove(&mut self, msg: &[u8]) -> Result<Proof> {
        self._prove(msg)
    }

    fn verify(&mut self, msg: &[u8], proof: &Proof) -> Result<bool> {
        self._verify(msg, proof)
    }

    fn check(&mut self, msg: &[u8], proof: &Proof) -> Result<()> {
       self._check(msg, proof)
    }
}

pub type Prover = HashCash;