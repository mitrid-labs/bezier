use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use bezier_lib::io::Session;
use bezier_lib::io::eval::StoreEvalResult;

#[test]
fn test_new_none() {
    let result = StoreEvalResult::new_none();
    assert_eq!(result, StoreEvalResult::None);
}

#[test]
fn test_new_path() {
    let path = String::from("path");
    
    let result = StoreEvalResult::new_path(&path);
    assert_eq!(result, StoreEvalResult::Path { path });
}

#[test]
fn test_new_sessions_prefix() {
    let prefix = b"prefix".to_vec();

    let result = StoreEvalResult::new_sessions_prefix(&prefix);
    assert_eq!(result, StoreEvalResult::SessionsPrefix { prefix });
}

#[test]
fn test_new_count_sessions() {
    let count = 10;

    let result = StoreEvalResult::new_count_sessions(count);
    assert_eq!(result, StoreEvalResult::CountSessions { count });
}

#[test]
fn test_new_list_sessions() {
    let sessions = vec![Session::default()];

    let res = StoreEvalResult::new_list_sessions(&sessions);
    assert!(res.is_ok());

    let result = res.unwrap();
    assert_eq!(result, StoreEvalResult::ListSessions { sessions });
}

#[test]
fn test_new_get_session() {
    let session = Session::default();

    let res = StoreEvalResult::new_get_session(&session);
    assert!(res.is_ok());

    let result = res.unwrap();
    assert_eq!(result, StoreEvalResult::GetSession { session });
}

#[test]
fn test_new_size() {
    let size = 10;

    let result = StoreEvalResult::new_size(size);
    assert_eq!(result, StoreEvalResult::Size { size });
}

#[test]
fn test_new_size_range() {
    let size = 10;

    let result = StoreEvalResult::new_size_range(size);
    assert_eq!(result, StoreEvalResult::SizeRange { size });
}

#[test]
fn test_new_size_prefix() {
    let size = 10;

    let result = StoreEvalResult::new_size_prefix(size);
    assert_eq!(result, StoreEvalResult::SizePrefix { size });
}

#[test]
fn test_new_dump() {
    let items = vec![(Vec::default(), Vec::default())];

    let result = StoreEvalResult::new_dump(&items);
    assert_eq!(result, StoreEvalResult::Dump { items });
}

#[test]
fn test_new_dump_range() {
    let items = vec![(Vec::default(), Vec::default())];

    let result = StoreEvalResult::new_dump_range(&items);
    assert_eq!(result, StoreEvalResult::DumpRange { items });
}

#[test]
fn test_new_dump_prefix() {
    let items = vec![(Vec::default(), Vec::default())];

    let result = StoreEvalResult::new_dump_prefix(&items);
    assert_eq!(result, StoreEvalResult::DumpPrefix { items });
}

#[test]
fn test_eval_result_default() {
    let result = StoreEvalResult::default();
    assert_eq!(result, StoreEvalResult::None);
}

#[test]
fn test_eval_result_size() {
    let result = StoreEvalResult::default();
    assert_eq!(result.size(), 0u8.size());

    let sessions = vec![Session::default()];

    let result = StoreEvalResult::new_list_sessions(&sessions).unwrap();

    let result_size = result.size();
    let expected_size = sessions.size();

    assert_eq!(result_size, expected_size);
}

#[test]
fn test_eval_result_check() {
    let result = StoreEvalResult::default();
    let res = result.check();
    assert!(res.is_ok());

    let sessions = vec![Session::default()];

    let result = StoreEvalResult::new_list_sessions(&sessions).unwrap();
    let res = result.check();
    assert!(res.is_ok());
}

#[test]
fn test_eval_result_serialize_json() {
    let result_a = StoreEvalResult::default();
    
    let res = result_a.to_json();
    assert!(res.is_ok());
    let json = res.unwrap();

    let res = StoreEvalResult::from_json(&json);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_eval_result_serialize_bytes() {
    let result_a = StoreEvalResult::default();
    
    let res = result_a.to_bytes();
    assert!(res.is_ok());
    let bytes = res.unwrap();

    let res = StoreEvalResult::from_bytes(&bytes);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_eval_result_serialize_hex() {
    let result_a = StoreEvalResult::default();
    
    let res = result_a.to_hex();
    assert!(res.is_ok());
    let hex = res.unwrap();

    let res = StoreEvalResult::from_hex(&hex);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}