use tempfile::tempdir;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::io::Store as StoreBase;
use mitrid_core::io::Permission;
use bezier_lib::io::Session;
use bezier_lib::io::Store;

use std::mem;

#[test]
fn test_store_open() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    temp_dir.close().unwrap();
}

#[test]
fn test_store_path() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let store = res.unwrap();

    let path = store.path();
    assert_eq!(path, store_path);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_destroy() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let store = res.unwrap();

    let res = store.destroy();
    assert!(res.is_ok());

    temp_dir.close().unwrap();
}

#[test]
fn test_store_size() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let res = store.size();
    assert!(res.is_ok());

    let mut size = res.unwrap();
    assert_eq!(size, 0);

    let session = Session::default();

    let prefix = Store::sessions_prefix();
    let key = Store::session_key(&session).unwrap();
    
    let mut store_key = Vec::new();
    store_key.extend_from_slice(&prefix);
    store_key.extend_from_slice(&key);

    let store_value = session.to_bytes().unwrap();
    
    let mut expected_size = store_key.size() + store_value.size();

    store.create_session(&session).unwrap();

    let res = store.size();
    assert!(res.is_ok());

    size = res.unwrap();
    assert_eq!(expected_size, size);

    let permission = Permission::Write;
    let session = store.session(&permission).unwrap();
    
    let mut store_key = Vec::new();
    store_key.extend_from_slice(&prefix);
    store_key.extend_from_slice(&key);

    let store_value = session.to_bytes().unwrap();
    
    expected_size += store_key.size() + store_value.size();

    let key = vec![1,1,1];
    let value = vec![1,1,1];

    expected_size += key.size() + value.size();

    store.create(&session, &key, &value).unwrap();

    let res = store.size();
    assert!(res.is_ok());

    size = res.unwrap();
    assert_eq!(expected_size, size);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_size_range() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let res = store.size();
    assert!(res.is_ok());

    let size = res.unwrap();
    assert_eq!(size, 0);

    let session = Session::default();

    let prefix = Store::sessions_prefix();
    
    let key = Store::session_key(&session).unwrap();
    
    let mut store_key = Vec::new();
    store_key.extend_from_slice(&prefix);
    store_key.extend_from_slice(&key);

    let store_value = session.to_bytes().unwrap();
    
    let mut sessions_size = store_key.size() + store_value.size();

    store.create_session(&session).unwrap();

    let permission = Permission::Write;
    let session = store.session(&permission).unwrap();
    
    let mut store_key = Vec::new();
    store_key.extend_from_slice(&prefix);
    store_key.extend_from_slice(&key);

    let store_value = session.to_bytes().unwrap();
    
    sessions_size += store_key.size() + store_value.size();

    let key = vec![0];
    let value = vec![1,1,1];

    let items_size = key.size() + value.size();

    store.create(&session, &key, &value).unwrap();

    let mut from = Some(vec![]);
    let mut to = Some(vec![]);

    let res = store.size_range(from.clone(), to.clone());
    assert!(res.is_err()); 

    from = Some(prefix.clone());
    to = None;

    let res = store.size_range(from.clone(), to.clone());
    assert!(res.is_ok());

    let size = res.unwrap();
    assert_eq!(sessions_size, size);

    from = None;
    to = Some(prefix);

    let res = store.size_range(from.clone(), to.clone());
    assert!(res.is_ok());

    let size = res.unwrap();
    assert_eq!(items_size, size);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_size_prefix() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let prefix = Store::sessions_prefix();

    let res = store.size_prefix(&prefix);
    assert!(res.is_ok());

    let mut size = res.unwrap();
    assert_eq!(size, 0);

    let session = Session::default();

    let key = Store::session_key(&session).unwrap();
    
    let mut store_key = Vec::new();
    store_key.extend_from_slice(&prefix);
    store_key.extend_from_slice(&key);

    let store_value = session.to_bytes().unwrap();
    
    let mut expected_size = store_key.size() + store_value.size();

    store.create_session(&session).unwrap();

    let res = store.size_prefix(&prefix);
    assert!(res.is_ok());

    size = res.unwrap();
    assert_eq!(expected_size, size);

    let permission = Permission::Write;
    let session = store.session(&permission).unwrap();
    
    let mut store_key = Vec::new();
    store_key.extend_from_slice(&prefix);
    store_key.extend_from_slice(&key);

    let store_value = session.to_bytes().unwrap();
    
    expected_size += store_key.size() + store_value.size();

    let key = vec![1,1,1];
    let value = vec![1,1,1];

    store.create(&session, &key, &value).unwrap();

    let res = store.size_prefix(&prefix);
    assert!(res.is_ok());

    size = res.unwrap();
    assert_eq!(expected_size, size);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_dump() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let permission = Permission::None;

    let count = 10;
    let mut expected_dump = Vec::new();

    let session_prefix = Store::sessions_prefix();

    for _ in 0..count {
        let session = store.session(&permission).unwrap();

        let key = Store::session_key(&session).unwrap();
        
        let mut store_key = Vec::new();
        store_key.extend_from_slice(&session_prefix);
        store_key.extend_from_slice(&key);

        let store_value = session.to_bytes().unwrap();

        expected_dump.push((store_key, store_value));
    }

    let permission = Permission::Write;

    let write_session = store.session(&permission).unwrap();

    let key = Store::session_key(&write_session).unwrap();
    
    let mut store_key = Vec::new();
    store_key.extend_from_slice(&session_prefix);
    store_key.extend_from_slice(&key);

    let store_value = write_session.to_bytes().unwrap();

    expected_dump.push((store_key, store_value));

    let key = vec![1,2,3];
    let value = vec![3,2,1];

    store.create(&write_session, &key, &value).unwrap();

    expected_dump.push((key, value));

    let permission = Permission::Read;

    let read_session = store.session(&permission).unwrap();

    let key = Store::session_key(&read_session).unwrap();
    
    let mut store_key = Vec::new();
    store_key.extend_from_slice(&session_prefix);
    store_key.extend_from_slice(&key);

    let store_value = read_session.to_bytes().unwrap();

    expected_dump.push((store_key, store_value));

    expected_dump.sort();

    let dump = store.dump().unwrap();
    assert_eq!(dump, expected_dump);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_dump_range() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let res = store.dump();
    assert!(res.is_ok());

    let dump = res.unwrap();
    assert_eq!(dump, vec![]);

    let session = Session::default();

    let prefix = Store::sessions_prefix();
    
    let key = Store::session_key(&session).unwrap();
    
    let mut store_key = Vec::new();
    store_key.extend_from_slice(&prefix);
    store_key.extend_from_slice(&key);

    let store_value = session.to_bytes().unwrap();

    let mut sessions_dump = Vec::new();
    
    sessions_dump.push((store_key, store_value));

    store.create_session(&session).unwrap();

    let permission = Permission::Write;
    let session = store.session(&permission).unwrap();
    
    let key = Store::session_key(&session).unwrap();
    
    let mut store_key = Vec::new();
    store_key.extend_from_slice(&prefix);
    store_key.extend_from_slice(&key);

    let store_value = session.to_bytes().unwrap();
    
    sessions_dump.push((store_key, store_value));

    sessions_dump.sort();

    let key = vec![0];
    let value = vec![1,1,1];

    store.create(&session, &key, &value).unwrap();

    let items_dump = vec![(key.clone(), value.clone())];

    let mut from = Some(vec![]);
    let mut to = Some(vec![]);

    let res = store.dump_range(from.clone(), to.clone());
    assert!(res.is_err()); 

    from = Some(prefix.clone());
    to = None;

    let res = store.dump_range(from.clone(), to.clone());
    assert!(res.is_ok());

    let dump = res.unwrap();
    assert_eq!(dump, sessions_dump);

    from = None;
    to = Some(prefix);

    let res = store.dump_range(from.clone(), to.clone());
    assert!(res.is_ok());

    let dump = res.unwrap();
    assert_eq!(dump, items_dump);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_dump_prefix() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let res = store.dump();
    assert!(res.is_ok());

    let dump = res.unwrap();
    assert_eq!(dump, vec![]);

    let session = Session::default();

    let prefix = Store::sessions_prefix();
    
    let key = Store::session_key(&session).unwrap();
    
    let mut store_key = Vec::new();
    store_key.extend_from_slice(&prefix);
    store_key.extend_from_slice(&key);

    let store_value = session.to_bytes().unwrap();

    let mut sessions_dump = Vec::new();
    
    sessions_dump.push((store_key, store_value));

    store.create_session(&session).unwrap();

    let permission = Permission::Write;
    let session = store.session(&permission).unwrap();
    
    let key = Store::session_key(&session).unwrap();
    
    let mut store_key = Vec::new();
    store_key.extend_from_slice(&prefix);
    store_key.extend_from_slice(&key);

    let store_value = session.to_bytes().unwrap();
    
    sessions_dump.push((store_key, store_value));

    sessions_dump.sort();

    let key = vec![0];
    let value = vec![1,1,1];

    store.create(&session, &key, &value).unwrap();

    let mut all_dump = vec![(key.clone(), value.clone())];
    all_dump.extend_from_slice(&sessions_dump);
    all_dump.sort();

    let res = store.dump_prefix(&prefix);
    assert!(res.is_ok());

    let dump = res.unwrap();
    assert_eq!(dump, sessions_dump);

    let res = store.dump_prefix(&key);
    assert!(res.is_ok());

    let dump = res.unwrap();
    assert_eq!(dump, all_dump);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_drop() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let permission = Permission::None;

    let count = 10;

    for _ in 0..count {
        let _ = store.session(&permission).unwrap();
    }

    let permission = Permission::Write;

    let write_session = store.session(&permission).unwrap();

    let key = vec![1,2,3];
    let value = vec![3,2,1];

    store.create(&write_session, &key, &value).unwrap();

    let permission = Permission::Read;

    let read_session = store.session(&permission).unwrap();

    let count = store.count(&read_session, None, None).unwrap();
    assert_eq!(count, 13);

    let res = store.drop();
    assert!(res.is_ok());

    let dump = store.dump().unwrap();
    assert_eq!(dump, vec![]);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_drop_range() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let res = store.dump();
    assert!(res.is_ok());

    let dump = res.unwrap();
    assert_eq!(dump, vec![]);

    let session = Session::default();

    store.create_session(&session).unwrap();

    let write_permission = Permission::Write;
    let write_session = store.session(&write_permission).unwrap();

    let read_permission = Permission::Read;
    let read_session = store.session(&read_permission).unwrap();

    let key = vec![0];
    let value = vec![1,1,1];

    store.create(&write_session, &key, &value).unwrap();

    let items_dump = vec![(key.clone(), value.clone())];

    let count = store.count(&read_session, None, None).unwrap();
    assert_eq!(count, 4);

    let mut from = Some(vec![]);
    let mut to = Some(vec![]);

    let res = store.drop_range(from.clone(), to.clone());
    assert!(res.is_err());

    let prefix = Store::sessions_prefix();

    from = Some(prefix.clone());
    to = None;

    let res = store.drop_range(from.clone(), to.clone());
    assert!(res.is_ok());

    let dump = store.dump().unwrap();
    assert_eq!(dump, items_dump);

    from = None;
    to = Some(prefix);

    let res = store.drop_range(from.clone(), to.clone());
    assert!(res.is_ok());

    let dump = store.dump().unwrap();
    assert_eq!(dump, vec![]);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_drop_prefix() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let res = store.dump();
    assert!(res.is_ok());

    let dump = res.unwrap();
    assert_eq!(dump, vec![]);

    let session = Session::default();

    store.create_session(&session).unwrap();

    let write_permission = Permission::Write;
    let write_session = store.session(&write_permission).unwrap();

    let read_permission = Permission::Read;
    let read_session = store.session(&read_permission).unwrap();

    let key = vec![0];
    let value = vec![1,1,1];

    store.create(&write_session, &key, &value).unwrap();

    let items_dump = vec![(key.clone(), value.clone())];

    let count = store.count(&read_session, None, None).unwrap();
    assert_eq!(count, 4);

    let prefix = Store::sessions_prefix();

    let res = store.drop_prefix(&prefix);
    assert!(res.is_ok());

    let dump = store.dump().unwrap();
    assert_eq!(dump, items_dump);

    let res = store.drop_prefix(b"");
    assert!(res.is_ok());

    let dump = store.dump().unwrap();
    assert_eq!(dump, vec![]);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_session_id() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let res = store.session_id();
    assert!(res.is_ok());

    let mut id = res.unwrap();
    assert_eq!(id, 0);

    let permission = Permission::Read;

    let mut session = store.session(&permission).unwrap();
    assert_eq!(session.id, id);

    let res = store.session_id();
    assert!(res.is_ok());

    id = res.unwrap();
    assert_eq!(id, 1);

    session = store.session(&permission).unwrap();
    assert_eq!(session.id, id);

    let res = store.session_id();
    assert!(res.is_ok());

    id = res.unwrap();
    assert_eq!(id, 2);

    session = store.session(&permission).unwrap();
    assert_eq!(session.id, id);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_session_key() {
    let session = Session::default();
    let id = session.id;

    let res = Store::session_key(&session);
    assert!(res.is_ok());

    let key = res.unwrap();

    let mut expected_key = Vec::new();
    let _id: [u8; 8] = unsafe { mem::transmute(id) };
    expected_key.extend_from_slice(&_id[..]);

    assert_eq!(expected_key, key);
}

#[test]
fn test_store_count_sessions() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let res = store.count_sessions();
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let permission = Permission::None;

    let expected_count = 10;

    for _ in 0..expected_count {
        let _ = store.session(&permission).unwrap();
    }

    let res = store.count_sessions();
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, expected_count);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_list_sessions() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let res = store.list_sessions();
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let permission = Permission::None;

    let len = 10;
    let mut expected_sessions = Vec::new();

    for _ in 0..len {
        let session = store.session(&permission).unwrap();
        expected_sessions.push(session);
    }

    expected_sessions.sort();

    let res = store.list_sessions();
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, expected_sessions);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_get_session() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let permission = Permission::None;

    let session = store.session(&permission).unwrap();

    let res = store.get_session(session.id);
    assert!(res.is_ok());

    let found_session = res.unwrap();
    assert_eq!(found_session, session);

    let res = store.get_session(session.id + 1);
    assert!(res.is_err());

    temp_dir.close().unwrap();
}

#[test]
fn test_store_create_session() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let res = store.list_sessions();
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let session = Session::default();

    store.create_session(&session).unwrap();

    let found_session = store.get_session(session.id).unwrap();
    assert_eq!(found_session, session);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_del_session() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let permission = Permission::None;

    let session = store.session(&permission).unwrap();

    let found_session = store.get_session(session.id).unwrap();
    assert_eq!(found_session, session);

    let res = store.del_session(session.id);
    assert!(res.is_ok());

    let res = store.get_session(session.id);
    assert!(res.is_err());

    temp_dir.close().unwrap();
}

#[test]
fn test_store_cleanup_sessions() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let permission = Permission::None;

    let count = 10;
    let mut valid_sessions = Vec::new();

    for _ in 0..count {
        let session = store.session(&permission).unwrap();
        valid_sessions.push(session);
    }

    valid_sessions.sort();

    let id = store.session_id().unwrap();

    for i in 0..count {
        let mut session = Session::default();
        
        let is_expired = session.is_expired().unwrap();
        assert!(is_expired);

        session.id = id + i;

        store.create_session(&session).unwrap();
    }

    let count = store.count_sessions().unwrap();
    assert_eq!(count, 20);

    let res = store.cleanup_sessions();
    assert!(res.is_ok());

    let count = store.count_sessions().unwrap();
    assert_eq!(count, 10);

    let list = store.list_sessions().unwrap();
    assert_eq!(list, valid_sessions);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_drop_sessions() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let permission = Permission::None;

    let count = 10;

    for _ in 0..count {
        let _ = store.session(&permission).unwrap();
    }

    let count = store.count_sessions().unwrap();
    assert_eq!(count, 10);

    let res = store.drop_sessions();
    assert!(res.is_ok());

    let count = store.count_sessions().unwrap();
    assert_eq!(count, 0);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_session() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let write_permission = Permission::Write;
    let read_permission = Permission::Read;

    let mut id = store.session_id().unwrap();

    let res = store.session(&write_permission);
    let write_session = res.unwrap();
    assert_eq!(write_session.id, id);
    assert_eq!(write_session.permission, write_permission);

    let res = write_session.is_expired();
    assert!(res.is_ok());
    assert!(!res.unwrap());

    id = store.session_id().unwrap();

    let res = store.session(&read_permission);
    let read_session = res.unwrap();
    assert_eq!(read_session.id, id);
    assert_eq!(read_session.permission, read_permission);

    let res = read_session.is_expired();
    assert!(res.is_ok());
    assert!(!res.unwrap());

    temp_dir.close().unwrap();
}

#[test]
fn test_store_count() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let write_permission = Permission::Write;
    let write_session = store.session(&write_permission).unwrap();

    let key = Vec::default();
    let value = Vec::default();

    store.create(&write_session, &key, &value).unwrap();

    let mut from = None;
    let mut to = None;

    let res = store.count(&write_session, from.clone(), to.clone());
    assert!(res.is_err());

    let read_permission = Permission::Read;
    let read_session = store.session(&read_permission).unwrap();

    let res = store.count(&read_session, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 3);

    from = Some(key.clone());
    to = Some(Store::sessions_prefix());

    let res = store.count(&read_session, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    to = Some(key.clone());

    let res = store.count(&read_session, from, to);
    assert!(res.is_err());

    temp_dir.close().unwrap();
}

#[test]
fn test_store_count_prefix() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let write_permission = Permission::Write;
    let write_session = store.session(&write_permission).unwrap();

    let key = Vec::default();
    let value = Vec::default();

    store.create(&write_session, &key, &value).unwrap();

    let prefix = Vec::new();

    let res = store.count_prefix(&write_session, &prefix);
    assert!(res.is_err());

    let read_permission = Permission::Read;
    let read_session = store.session(&read_permission).unwrap();

    let res = store.count_prefix(&read_session, &prefix);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 3);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_list() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let write_permission = Permission::Write;
    let write_session = store.session(&write_permission).unwrap();
    let write_session_value = write_session.to_bytes().unwrap();

    let key = Vec::default();
    let value = Vec::default();

    store.create(&write_session, &key, &value).unwrap();

    let mut from = None;
    let mut to = None;
    let mut count = None;
    let mut skip = 0;

    let res = store.list(&write_session, from.clone(), to.clone(), count, skip);
    assert!(res.is_err());

    let read_permission = Permission::Read;
    let read_session = store.session(&read_permission).unwrap();
    let read_session_value = read_session.to_bytes().unwrap();

    let res = store.list(&read_session, from.clone(), to.clone(), count, skip);
    assert!(res.is_ok());

    let mut values = vec![write_session_value, read_session_value, value];
    values.sort();

    let list = res.unwrap();
    assert_eq!(&list, &values);

    from = Some(key.clone());

    let res = store.list(&read_session, from.clone(), to.clone(), count, skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(&list, &values);

    to = Some(key.clone());

    let res = store.list(&read_session, from.clone(), to.clone(), count, skip);
    assert!(res.is_err());

    from = None;
    to = None;
    count = Some(1);
    skip = 2;

    let res = store.list(&read_session, from.clone(), to.clone(), count, skip);
    assert!(res.is_err());

    skip = 0;

    let res = store.list(&read_session, from.clone(), to.clone(), count, skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    let first = values.first().unwrap().to_owned();
    assert_eq!(&list, &vec![first]);

    skip = 1;

    let res = store.list(&read_session, from.clone(), to.clone(), count, skip);
    assert!(res.is_err());

    count = Some(0);

    let res = store.list(&read_session, from, to, count, skip);
    assert!(res.is_err());

    temp_dir.close().unwrap();
}

#[test]
fn test_store_list_prefix() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let write_permission = Permission::Write;
    let write_session = store.session(&write_permission).unwrap();
    let write_session_value = write_session.to_bytes().unwrap();

    let key = Vec::default();
    let value = Vec::default();

    store.create(&write_session, &key, &value).unwrap();

    let prefix = Vec::new();
    let mut count = None;
    let mut skip = 0;

    let res = store.list_prefix(&write_session, &prefix, count, skip);
    assert!(res.is_err());

    let read_permission = Permission::Read;
    let read_session = store.session(&read_permission).unwrap();
    let read_session_value = read_session.to_bytes().unwrap();

    let res = store.list_prefix(&read_session, &prefix, count, skip);
    assert!(res.is_ok());

    let mut values = vec![write_session_value, read_session_value, value];
    values.sort();

    let list = res.unwrap();
    assert_eq!(&list, &values);

    count = Some(1);
    skip = 2;

    let res = store.list_prefix(&read_session, &prefix, count, skip);
    assert!(res.is_err());

    skip = 1;

    let res = store.list_prefix(&read_session, &prefix, count, skip);
    assert!(res.is_err());

    count = Some(0);
    skip = 0;

    let res = store.list_prefix(&read_session, &prefix, count, skip);
    assert!(res.is_err());

    temp_dir.close().unwrap();
}

#[test]
fn test_store_lookup() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let write_permission = Permission::Write;
    let write_session = store.session(&write_permission).unwrap();

    let mut key = Vec::default();
    let value = Vec::default();

    store.create(&write_session, &key, &value).unwrap();

    let res = store.lookup(&write_session, &key);
    assert!(res.is_err());

    let read_permission = Permission::Read;
    let read_session = store.session(&read_permission).unwrap();

    let res = store.lookup(&read_session, &key);
    assert!(res.is_ok());
    assert!(res.unwrap());

    key.push(1);

    let res = store.lookup(&read_session, &key);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    temp_dir.close().unwrap();
}

#[test]
fn test_store_get() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let write_permission = Permission::Write;
    let write_session = store.session(&write_permission).unwrap();

    let mut key = Vec::default();
    let value = Vec::default();

    store.create(&write_session, &key, &value).unwrap();

    let res = store.get(&write_session, &key);
    assert!(res.is_err());

    let read_permission = Permission::Read;
    let read_session = store.session(&read_permission).unwrap();

    let res = store.get(&read_session, &key);
    assert!(res.is_ok());
    
    let found_value = res.unwrap();
    assert_eq!(found_value, value);

    key.push(1);

    let res = store.get(&read_session, &key);
    assert!(res.is_err());

    temp_dir.close().unwrap();
}

#[test]
fn test_store_create() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let key = Vec::default();
    let value = Vec::default();

    let read_permission = Permission::Read;
    let read_session = store.session(&read_permission).unwrap();

    let res = store.create(&read_session, &key, &value);
    assert!(res.is_err());

    let write_permission = Permission::Write;
    let write_session = store.session(&write_permission).unwrap();

    let res = store.create(&write_session, &key, &value);
    assert!(res.is_ok());

    temp_dir.close().unwrap();
}

#[test]
fn test_store_update() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let write_permission = Permission::Write;
    let write_session = store.session(&write_permission).unwrap();

    let mut key = Vec::default();
    let mut value = Vec::default();

    store.create(&write_session, &key, &value).unwrap();

    let read_permission = Permission::Read;
    let read_session = store.session(&read_permission).unwrap();

    let found_value = store.get(&read_session, &key).unwrap();
    assert_eq!(found_value, value);

    value.push(1);

    let res = store.update(&read_session, &key, &value);
    assert!(res.is_err());

    let res = store.update(&write_session, &key, &value);
    assert!(res.is_ok());

    let found_value = store.get(&read_session, &key).unwrap();
    assert_eq!(found_value, value);

    key.push(1);

    let res = store.update(&write_session, &key, &value);
    assert!(res.is_err());

    temp_dir.close().unwrap();
}

#[test]
fn test_store_upsert() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let write_permission = Permission::Write;
    let write_session = store.session(&write_permission).unwrap();

    let mut key = Vec::default();
    let mut value = Vec::default();

    store.create(&write_session, &key, &value).unwrap();

    let read_permission = Permission::Read;
    let read_session = store.session(&read_permission).unwrap();

    let found_value = store.get(&read_session, &key).unwrap();
    assert_eq!(found_value, value);

    value.push(1);

    let res = store.upsert(&read_session, &key, &value);
    assert!(res.is_err());

    let res = store.upsert(&write_session, &key, &value);
    assert!(res.is_ok());

    let found_value = store.get(&read_session, &key).unwrap();
    assert_eq!(found_value, value);

    key.push(1);

    let res = store.upsert(&write_session, &key, &value);
    assert!(res.is_ok());

    let found_value = store.get(&read_session, &key).unwrap();
    assert_eq!(found_value, value);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_delete() {
    let store_file_name = "bezier.store.db";
    let temp_dir = tempdir().unwrap();

    let store_path = format!("{}", temp_dir.path().join(store_file_name).to_str().unwrap());

    let res = Store::open(&store_path);
    assert!(res.is_ok());

    let mut store = res.unwrap();

    let write_permission = Permission::Write;
    let write_session = store.session(&write_permission).unwrap();

    let key = Vec::default();
    let value = Vec::default();

    store.create(&write_session, &key, &value).unwrap();

    let read_permission = Permission::Read;
    let read_session = store.session(&read_permission).unwrap();

    let found = store.lookup(&read_session, &key).unwrap();
    assert!(found);

    let res = store.delete(&read_session, &key);
    assert!(res.is_err());

    let res = store.delete(&write_session, &key);
    assert!(res.is_ok());

    let found = store.lookup(&read_session, &key).unwrap();
    assert!(!found);

    temp_dir.close().unwrap();
}

#[test]
fn test_store_eval_none() {}

#[test]
fn test_store_eval_path() {}

#[test]
fn test_store_eval_sessions_prefix() {}

#[test]
fn test_store_eval_count_sessions() {}

#[test]
fn test_store_eval_list_sessions() {}

#[test]
fn test_store_eval_get_session() {}

#[test]
fn test_store_eval_size() {}

#[test]
fn test_store_eval_size_range() {}

#[test]
fn test_store_eval_size_prefix() {}

#[test]
fn test_store_eval_dump() {}

#[test]
fn test_store_eval_dump_range() {}

#[test]
fn test_store_eval_dump_prefix() {}

#[test]
fn test_store_eval_mut_none() {}

#[test]
fn test_store_eval_mut_drop() {}

#[test]
fn test_store_eval_mut_drop_range() {}

#[test]
fn test_store_eval_mut_drop_prefix() {}