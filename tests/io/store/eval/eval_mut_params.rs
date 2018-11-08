use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use bezier_lib::io::eval::StoreEvalMutParams;

#[test]
fn test_new_none() {
    let params = StoreEvalMutParams::new_none();
    assert_eq!(params, StoreEvalMutParams::None);
}

#[test]
fn test_new_drop() {
    let params = StoreEvalMutParams::new_drop();
    assert_eq!(params, StoreEvalMutParams::Drop);
}

#[test]
fn test_new_drop_range() {
    let prefix = Vec::new();
    let from = Some(vec![]);
    let mut to = Some(vec![]);

    let res = StoreEvalMutParams::new_drop_range(&prefix, from.clone(), to.clone());
    assert!(res.is_err());

    to = Some(vec![1]);

    let res = StoreEvalMutParams::new_drop_range(&prefix, from.clone(), to.clone());
    assert!(res.is_ok());

    let params = res.unwrap();
    assert_eq!(params, StoreEvalMutParams::DropRange { prefix, from, to });
}

#[test]
fn test_new_drop_prefix() {
    let prefix = Vec::new();

    let params = StoreEvalMutParams::new_drop_prefix(&prefix);
    assert_eq!(params, StoreEvalMutParams::DropPrefix { prefix });
}

#[test]
fn test_eval_mut_default() {
    let params = StoreEvalMutParams::default();
    assert_eq!(params, StoreEvalMutParams::None);
}

#[test]
fn test_eval_mut_params_size() {
    let params = StoreEvalMutParams::default();
    assert_eq!(params.size(), 0u8.size());

    let prefix = Vec::new();
    let from = None;
    let to = Some(vec![]);

    let params = StoreEvalMutParams::new_drop_range(&prefix, from.clone(), to.clone()).unwrap();

    let params_size = params.size();
    let mut expected_size = 0;

    match params {
        StoreEvalMutParams::DropRange { ref prefix, ref from, ref to } => {
            expected_size += prefix.size() + from.size() + to.size();
        },
        _ => panic!("invalid params"),
    }

    assert_eq!(params_size, expected_size);
}

#[test]
fn test_eval_mut_params_check() {
    let params = StoreEvalMutParams::default();
    let res = params.check();
    assert!(res.is_ok());

    let prefix = Vec::new();
    let mut from = None;
    let to = Some(vec![]);

    let params = StoreEvalMutParams::new_drop_range(&prefix, from.clone(), to.clone()).unwrap();
    let res = params.check();
    assert!(res.is_ok());

    from = Some(vec![]);

    let params = StoreEvalMutParams::DropRange { prefix, from, to };
    let res = params.check();
    assert!(res.is_err());
}

#[test]
fn test_eval_mut_params_serialize_json() {
    let params_a = StoreEvalMutParams::default();
    
    let res = params_a.to_json();
    assert!(res.is_ok());
    let json = res.unwrap();

    let res = StoreEvalMutParams::from_json(&json);
    assert!(res.is_ok());
    let params_b = res.unwrap();

    assert_eq!(params_a, params_b);
}

#[test]
fn test_eval_mut_params_serialize_bytes() {
    let params_a = StoreEvalMutParams::default();
    
    let res = params_a.to_bytes();
    assert!(res.is_ok());
    let bytes = res.unwrap();

    let res = StoreEvalMutParams::from_bytes(&bytes);
    assert!(res.is_ok());
    let params_b = res.unwrap();

    assert_eq!(params_a, params_b);
}

#[test]
fn test_eval_mut_params_serialize_hex() {
    let params_a = StoreEvalMutParams::default();
    
    let res = params_a.to_hex();
    assert!(res.is_ok());
    let hex = res.unwrap();

    let res = StoreEvalMutParams::from_hex(&hex);
    assert!(res.is_ok());
    let params_b = res.unwrap();

    assert_eq!(params_a, params_b);
}