use mitrid_core::base::Result;
use mitrid_core::base::{Sizable, ConstantSize};
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

use crypto::{Digest, SecretKey};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct WalletPayload {
    pub coin_keys_len: u64,
    pub coin_keys: Vec<(Digest, SecretKey)>,
}

impl WalletPayload {
    pub fn new() -> WalletPayload {
        WalletPayload::default()
    }

    pub fn lookup_coin_key(&self, coin_id: &Digest) -> bool {
        let res = self.coin_keys.binary_search_by(|coin_key| {
                    coin_key.0.cmp(&coin_id)
                  });

        res.is_ok()
    }

    pub fn get_coin_key(&self, coin_id: &Digest) -> Result<(Digest, SecretKey)> {
        let idx = self.coin_keys.binary_search_by(|coin_key| {
            coin_key.0.cmp(coin_id)
        }).map_err(|e| format!("{:?}", e))?;

        let coin_key = self.coin_keys[idx].clone();

        Ok(coin_key)
    }

    pub fn add_coin_key(&mut self, coin_key: &(Digest, SecretKey)) -> Result<()> {
        self.check()?;

        coin_key.0.check()?;
        coin_key.0.check_size()?;
        coin_key.1.check()?;
        coin_key.1.check_size()?;

        if self.lookup_coin_key(&coin_key.0) {
            return Err(format!("already found"));
        }

        self.coin_keys.push(coin_key.to_owned());
        self.coin_keys_len += 1;

        Ok(())
    }

    pub fn del_coin_key_from_coin_id(&mut self, coin_id: &Digest) -> Result<()> {
        let idx = self.coin_keys.binary_search_by(|coin_key| {
            coin_key.0.cmp(coin_id)
        }).map_err(|e| format!("{:?}", e))?;

        self.coin_keys.remove(idx);
        self.coin_keys_len -= 1;

        Ok(())
    }

    pub fn del_coin_key_from_coin_sk(&mut self, coin_sk: &SecretKey) -> Result<()> {
        let idx = self.coin_keys.binary_search_by(|coin_key| {
            coin_key.1.cmp(coin_sk)
        }).map_err(|e| format!("{:?}", e))?;

        self.coin_keys.remove(idx);
        self.coin_keys_len -= 1;

        Ok(())
    }

    pub fn del_coin_key(&mut self, coin_key: &(Digest, SecretKey)) -> Result<()> {
        let idx = self.coin_keys.binary_search_by(|_coin_key| {
            _coin_key.cmp(coin_key)
        }).map_err(|e| format!("{:?}", e))?;

        self.coin_keys.remove(idx);
        self.coin_keys_len -= 1;

        Ok(())
    }
}

impl Sizable for WalletPayload {
    fn size(&self) -> u64 {
        self.coin_keys_len.size() +
        self.coin_keys.size()
    }
}

impl Checkable for WalletPayload {
    fn check(&self) -> Result<()> {
        self.coin_keys_len.check()?;
        self.coin_keys.check()?;

        for coin_key in self.coin_keys.iter() {
            coin_key.0.check()?;
            coin_key.0.check_size()?;
            coin_key.1.check()?;
            coin_key.1.check_size()?;
        }

        if self.coin_keys_len != self.coin_keys.len() as u64 {
            return Err(format!("invalid length"));
        }

        let mut unique_coin_keys = self.coin_keys.clone();
        unique_coin_keys.dedup_by(|a, b| { a.0 == b.0 });

        if unique_coin_keys.len() != self.coin_keys.len() {
            return Err(format!("duplicates found"));
        }

        Ok(())
    }
}

impl Serializable for WalletPayload {}

impl Datable for WalletPayload {}