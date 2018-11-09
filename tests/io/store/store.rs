use tempfile::tempdir;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::io::Store as StoreBase;
use mitrid_core::io::Permission;
use bezier_lib::io::Session;
use bezier_lib::io::Store;

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
}

#[test]
fn test_store_dump() {}

#[test]
fn test_store_dump_range() {}

#[test]
fn test_store_dump_prefix() {}

#[test]
fn test_store_drop() {}

#[test]
fn test_store_drop_range() {}

#[test]
fn test_store_drop_prefix() {}

#[test]
fn test_store_session_id() {}

#[test]
fn test_store_session_key() {}

#[test]
fn test_store_count_sessions() {}

#[test]
fn test_store_list_sessions() {}

#[test]
fn test_store_get_session() {}

#[test]
fn test_store_create_session() {}

#[test]
fn test_store_del_session() {}

#[test]
fn test_store_cleanup_sessions() {}

#[test]
fn test_store_drop_sessions() {}

#[test]
fn test_store_session() {}

#[test]
fn test_store_count() {}

#[test]
fn test_store_list() {}

#[test]
fn test_store_lookup() {}

#[test]
fn test_store_get() {}

#[test]
fn test_store_create() {}

#[test]
fn test_store_update() {}

#[test]
fn test_store_upsert() {}

#[test]
fn test_store_delete() {}

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