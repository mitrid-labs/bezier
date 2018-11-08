use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use bezier_lib::io::eval::StoreEvalParams;

#[test]
fn test_new_none() {
    let params = StoreEvalParams::new_none();
    assert_eq!(params, StoreEvalParams::None);
}

#[test]
fn test_new_path() {
    let params = StoreEvalParams::new_path();
    assert_eq!(params, StoreEvalParams::Path);
}

#[test]
fn test_new_sessions_prefix() {
    let params = StoreEvalParams::new_sessions_prefix();
    assert_eq!(params, StoreEvalParams::SessionsPrefix);
}

#[test]
fn test_new_count_sessions() {
    let params = StoreEvalParams::new_count_sessions();
    assert_eq!(params, StoreEvalParams::CountSessions);
}

#[test]
fn test_new_list_sessions() {
    let params = StoreEvalParams::new_list_sessions();
    assert_eq!(params, StoreEvalParams::ListSessions);
}

#[test]
fn test_new_get_session() {
    let id = 0;

    let params = StoreEvalParams::new_get_session(id);
    assert_eq!(params, StoreEvalParams::GetSession { id });
}

#[test]
fn test_new_size() {
    let params = StoreEvalParams::new_size();
    assert_eq!(params, StoreEvalParams::Size);
}

#[test]
fn test_new_size_range() {
    let prefix = Vec::new();
    let from = Some(vec![]);
    let mut to = Some(vec![]);

    let res = StoreEvalParams::new_size_range(&prefix, from.clone(), to.clone());
    assert!(res.is_err());

    to = Some(vec![1]);

    let res = StoreEvalParams::new_size_range(&prefix, from.clone(), to.clone());
    assert!(res.is_ok());

    let params = res.unwrap();
    assert_eq!(params, StoreEvalParams::SizeRange { prefix, from, to });
}

#[test]
fn test_new_size_prefix() {
    let prefix = Vec::new();

    let params = StoreEvalParams::new_size_prefix(&prefix);
    assert_eq!(params, StoreEvalParams::SizePrefix { prefix });
}

#[test]
fn test_new_dump() {
    let params = StoreEvalParams::new_dump();
    assert_eq!(params, StoreEvalParams::Dump);
}

#[test]
fn test_new_dump_range() {
    let prefix = Vec::new();
    let from = Some(vec![]);
    let mut to = Some(vec![]);

    let res = StoreEvalParams::new_dump_range(&prefix, from.clone(), to.clone());
    assert!(res.is_err());

    to = Some(vec![1]);

    let res = StoreEvalParams::new_dump_range(&prefix, from.clone(), to.clone());
    assert!(res.is_ok());

    let params = res.unwrap();
    assert_eq!(params, StoreEvalParams::DumpRange { prefix, from, to });
}

#[test]
fn test_new_dump_prefix() {
    let prefix = Vec::new();

    let params = StoreEvalParams::new_dump_prefix(&prefix);
    assert_eq!(params, StoreEvalParams::DumpPrefix { prefix });
}

#[test]
fn test_eval_params_default() {
    let params = StoreEvalParams::default();
    assert_eq!(params, StoreEvalParams::None);
}

#[test]
fn test_eval_params_size() {
    let params = StoreEvalParams::default();
    assert_eq!(params.size(), 0u8.size());

    let prefix = Vec::new();
    let from = None;
    let to = Some(vec![]);

    let params = StoreEvalParams::new_dump_range(&prefix, from.clone(), to.clone()).unwrap();

    let params_size = params.size();
    let mut expected_size = 0;

    match params {
        StoreEvalParams::DumpRange { ref prefix, ref from, ref to } => {
            expected_size += prefix.size() + from.size() + to.size();
        },
        _ => panic!("invalid params"),
    }

    assert_eq!(params_size, expected_size);
}

#[test]
fn test_eval_params_check() {
    let params = StoreEvalParams::default();
    let res = params.check();
    assert!(res.is_ok());

    let prefix = Vec::new();
    let mut from = None;
    let to = Some(vec![]);

    let params = StoreEvalParams::new_dump_range(&prefix, from.clone(), to.clone()).unwrap();
    let res = params.check();
    assert!(res.is_ok());

    from = Some(vec![]);

    let params = StoreEvalParams::DumpRange { prefix, from, to };
    let res = params.check();
    assert!(res.is_err());
}

#[test]
fn test_eval_params_serialize_json() {
    let params_a = StoreEvalParams::default();
    
    let res = params_a.to_json();
    assert!(res.is_ok());
    let json = res.unwrap();

    let res = StoreEvalParams::from_json(&json);
    assert!(res.is_ok());
    let params_b = res.unwrap();

    assert_eq!(params_a, params_b);
}

#[test]
fn test_eval_params_serialize_bytes() {
    let params_a = StoreEvalParams::default();
    
    let res = params_a.to_bytes();
    assert!(res.is_ok());
    let bytes = res.unwrap();

    let res = StoreEvalParams::from_bytes(&bytes);
    assert!(res.is_ok());
    let params_b = res.unwrap();

    assert_eq!(params_a, params_b);
}

#[test]
fn test_eval_params_serialize_hex() {
    let params_a = StoreEvalParams::default();
    
    let res = params_a.to_hex();
    assert!(res.is_ok());
    let hex = res.unwrap();

    let res = StoreEvalParams::from_hex(&hex);
    assert!(res.is_ok());
    let params_b = res.unwrap();

    assert_eq!(params_a, params_b);
}