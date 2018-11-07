use rocksdb::DB as RocksDB;
use rocksdb::Options;
use rocksdb::{IteratorMode, Direction};
use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::util::{Timestamp, TimestampDiff};
use mitrid_core::io::Permission;
use mitrid_core::io::Store as StoreBase;

use std::mem;

use io::Session;

pub const SESSION_DURATION: u64 = 3600; // 1 hour

pub struct Store {
    path: String,
    internal: RocksDB
}

impl Store {
    pub fn open(path: &str) -> Result<Store> {
        let internal = RocksDB::open_default(path)
            .map_err(|e| format!("{:?}", e))?;

        let store = Store {
            path: path.into(),
            internal: internal,
        };

        Ok(store)
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn repair_path(path: &str) -> Result<()> {
        RocksDB::repair(Options::default(), path)
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    pub fn repair(self) -> Result<Store> {
        let path = self.path();

        Self::repair_path(&path)?;

        Ok(self)
    }

    pub fn destroy_path(path: &str) -> Result<()> {
        RocksDB::destroy(&Options::default(), path)
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    pub fn destroy(self) -> Result<()> {
        let path = self.path();

        Self::destroy_path(&path)?;

        drop(self);

        Ok(())
    }

    fn _count(&self,
              from: Option<Vec<u8>>,
              to: Option<Vec<u8>>)
        -> Result<u64>
    {
        if let Some(ref from) = from {
            if let Some(ref to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        let mode = if let Some(ref from) = from {
            IteratorMode::From(&from, Direction::Forward)
        } else {
            IteratorMode::Start
        };

        let mut count = 0;

        for (key, _) in self.internal.iterator(mode) {
            if let Some(ref to) = to {
                if &*key < to {
                    count += 1;
                } else {
                    break;
                }
            }
        }

        Ok(count)
    }

    fn _count_prefix(&self,
                     prefix: &[u8])
        -> Result<u64>
    {
        let mut count = 0;

        for _ in self.internal.prefix_iterator(prefix) {
            count += 1;
        }

        Ok(count)
    }

    fn _list(&self,
             from: Option<Vec<u8>>,
             to: Option<Vec<u8>>,
             count: Option<u64>)
        -> Result<Vec<Vec<u8>>>
    {

        if let Some(ref from) = from {
            if let Some(ref to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        if let Some(count) = count {
            if count == 0 {
                return Err(String::from("invalid count"));
            }
        }

        let mode = if let Some(ref from) = from {
            IteratorMode::From(&from, Direction::Forward)
        } else {
            IteratorMode::Start
        };

        let mut list = Vec::new();
        let mut ct = 0;

        for (key, value) in self.internal.iterator(mode) {
            if let Some(ref to) = to {
                if &*key < to {
                    if let Some(count) = count {
                        if ct >= count {
                            break;
                        }
                    }

                    list.push((&*value).into());
                    ct += 1;
                } else {
                    break;
                }
            }
        }

        Ok(list)
    }

    fn _list_prefix(&self,
                    prefix: &[u8],
                    count: Option<u64>)
        -> Result<Vec<Vec<u8>>>
    {
        let mut list = Vec::new();
        let mut ct = 0;

        for (_, value) in self.internal.prefix_iterator(prefix) {
            if let Some(count) = count {
                if ct >= count { break; }

                list.push((&*value).to_vec());
                ct += 1;
            }
        }

        Ok(list)
    }

    fn _lookup(&self, key: &[u8]) -> Result<bool> {
        self._get(key)
            .map(|_| true)
            .or_else(|e| {
                if e == format!("not found") {
                    Ok(true)
                } else {
                    Err(e)
                }
            })
    }

    fn _get(&self, key: &[u8]) -> Result<Vec<u8>> {
        self.internal.get(key)
            .map_err(|e| format!("{:?}", e))
            .and_then(|opt_v| {
                match opt_v {
                    Some(v) => Ok((&*v).to_vec()),
                    None => Err(format!("not found")),
                }
            })
    }

    fn _create(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        if self._lookup(key)? {
            return Err(format!("already found"));
        }

        self._upsert(key, value)
    }

    fn _update(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        if !self._lookup(key)? {
            return Err(format!("not found"));
        }

        self._upsert(key, value)
    }

    fn _upsert(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        self.internal.put(key, value)
            .map_err(|e| format!("{:?}", e))
    }

    fn _delete(&mut self, key: &[u8]) -> Result<()> {
        self.internal.delete(key)
            .map_err(|e| format!("{:?}", e))
    }

    pub fn session_prefix() -> u64 {
        u64::max_value()
    }

    pub fn session_id(&self) -> Result<u64> {
        let count = self.count_sessions()?;
        
        let id = count + 1;
    
        Ok(id)
    }

    fn session_key_from_id(id: u64) -> Vec<u8> {
        let mut key = Vec::new();
        let _key: [u8; 8] = unsafe { mem::transmute(id) };
        key.extend_from_slice(&_key[..]);

        key
    }

    pub fn drop(&mut self) -> Result<()> {
        self.drop_prefix(b"")
    }

    pub fn drop_range(&mut self, from: Option<Vec<u8>>, to: Option<Vec<u8>>) -> Result<()> {
        if let Some(ref from) = from {
            if let Some(ref to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        let mode = if let Some(ref from) = from {
            IteratorMode::From(&from, Direction::Forward)
        } else {
            IteratorMode::Start
        };

        for (key, _) in self.internal.iterator(mode) {
            self._delete(&*key)?;
        }

        Ok(())
    }

    pub fn drop_prefix(&mut self, prefix: &[u8]) -> Result<()> {
        for (key, _) in self.internal.prefix_iterator(&prefix) {
            self._delete(&*key)?;
        }

        Ok(())
    }

    pub fn session_key(session: &Session) -> Result<Vec<u8>> {
        session.check()?;

        let key = Self::session_key_from_id(session.id);

        Ok(key)
    }

    pub fn count_sessions(&self) -> Result<u64> {
        let mut prefix = Vec::new();
        let _prefix: [u8; 8] = unsafe { mem::transmute(Self::session_prefix()) };
        prefix.extend_from_slice(&_prefix[..]);

        let mut count = 0;

        for _ in self.internal.prefix_iterator(&prefix) {
            count += 1;
        }

        Ok(count)
    }

    pub fn list_sessions(&self) -> Result<Vec<Session>> {
        let mut prefix = Vec::new();
        let _prefix: [u8; 8] = unsafe { mem::transmute(Self::session_prefix()) };
        prefix.extend_from_slice(&_prefix[..]);

        let mut list = Vec::new();

        for (_, value) in self.internal.prefix_iterator(&prefix) {
            let session = Session::from_bytes(&*value)?;
            list.push(session)
        }

        Ok(list)
    }

    pub fn get_session(&self, id: u64) -> Result<Session> {
        let key = Self::session_key_from_id(id);

        let value = self._get(&key)?;

        Session::from_bytes(&value)
    }

    pub fn create_session(&mut self, session: &Session) -> Result<()> {
        session.check()?;

        let key = Self::session_key_from_id(session.id);
        let value = session.to_bytes()?;

        self._create(&key, &value)
    }

    pub fn del_session(&mut self, id: u64) -> Result<()> {
        let key = Self::session_key_from_id(id);

        self._delete(&key)
    }

    pub fn cleanup_sessions(&mut self) -> Result<()> {
        let mut prefix = Vec::new();
        let _prefix: [u8; 8] = unsafe { mem::transmute(Self::session_prefix()) };
        prefix.extend_from_slice(&_prefix[..]);

        for (_, value) in self.internal.prefix_iterator(&prefix) {
            let session = Session::from_bytes(&*value)?;
            if session.is_expired()? {
                self.del_session(session.id)?;
            }
        }

        Ok(())
    }

    pub fn drop_sessions(&mut self) -> Result<()> {
        let mut prefix = Vec::new();
        let _prefix: [u8; 8] = unsafe { mem::transmute(Self::session_prefix()) };
        prefix.extend_from_slice(&_prefix[..]);

        for (_, value) in self.internal.prefix_iterator(&prefix) {
            let session = Session::from_bytes(&*value)?;
            self.del_session(session.id)?;
        }

        Ok(())
    }
}

impl Checkable for Store {}

impl StoreBase<()> for Store {
    fn session(&mut self, permission: &Permission) -> Result<Session> {
        permission.check()?;

        let mut expires_at = Timestamp::now()?;
        let duration = TimestampDiff::from_secs(SESSION_DURATION);
        expires_at += duration;

        let id = self.session_id()?;
        let session = Session::new(id, permission, &expires_at, &())?;
        self.create_session(&session)?;

        Ok(session)
    }
    
    fn count(&mut self,
             session: &Session,
             from: Option<Vec<u8>>,
             to: Option<Vec<u8>>)
        -> Result<u64>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        from.check()?;
        to.check()?;

        if let Some(ref from) = from {
            if let Some(ref to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        let found_session = self.get_session(session.id)?;
        if &found_session != session {
            return Err(String::from("session not found"));
        }

        self._count(from, to)
    }
    
    fn count_prefix(&mut self,
                    session: &Session,
                    prefix: &[u8])
        -> Result<u64>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        let found_session = self.get_session(session.id)?;
        if &found_session != session {
            return Err(String::from("session not found"));
        }

        self._count_prefix(prefix)
    }
    
    fn list(&mut self,
            session: &Session,
            from: Option<Vec<u8>>,
            to: Option<Vec<u8>>,
            count: Option<u64>)
        -> Result<Vec<Vec<u8>>>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        from.check()?;
        to.check()?;
        count.check()?;

        if let Some(ref from) = from {
            if let Some(ref to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        if let Some(count) = count {
            if count == 0 {
                return Err(String::from("invalid count"));
            }
        }

        let found_session = self.get_session(session.id)?;
        if &found_session != session {
            return Err(String::from("session not found"));
        }

        self._list(from, to, count)
    }
    
    fn list_prefix(&mut self,
                   session: &Session,
                   prefix: &[u8],
                   count: Option<u64>)
        -> Result<Vec<Vec<u8>>>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        count.check()?;

        if let Some(count) = count {
            if count == 0 {
                return Err(String::from("invalid count"));
            }
        }

        let found_session = self.get_session(session.id)?;
        if &found_session != session {
            return Err(String::from("session not found"));
        }

        self._list_prefix(prefix, count)
    }
    
    fn lookup(&mut self,
              session: &Session,
              key: &[u8])
        -> Result<bool>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        let found_session = self.get_session(session.id)?;
        if &found_session != session {
            return Err(String::from("session not found"));
        }

        self._lookup(key)
    }
    
    fn get(&mut self,
           session: &Session,
           key: &[u8])
        -> Result<Vec<u8>>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        let found_session = self.get_session(session.id)?;
        if &found_session != session {
            return Err(String::from("session not found"));
        }

        self._get(key)
    }
    
    fn create(&mut self,
              session: &Session,
              key: &[u8],
              value: &[u8])
        -> Result<()>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        let found_session = self.get_session(session.id)?;
        if &found_session != session {
            return Err(String::from("session not found"));
        }

        self._create(key, value)
    }
    
    fn update(&mut self,
              session: &Session,
              key: &[u8],
              value: &[u8])
        -> Result<()>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        let found_session = self.get_session(session.id)?;
        if &found_session != session {
            return Err(String::from("session not found"));
        }

        self._update(key, value)
    }
    
    fn upsert(&mut self,
              session: &Session,
              key: &[u8],
              value: &[u8])
        -> Result<()>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        let found_session = self.get_session(session.id)?;
        if &found_session != session {
            return Err(String::from("session not found"));
        }

        self._upsert(key, value)
    }
    
    fn delete(&mut self,
              session: &Session,
              key: &[u8])
        -> Result<()>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        let found_session = self.get_session(session.id)?;
        if &found_session != session {
            return Err(String::from("session not found"));
        }

        self._delete(key)
    }
}