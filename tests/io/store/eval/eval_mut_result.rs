use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use bezier_lib::io::eval::StoreEvalMutResult;

#[test]
fn test_eval_mut_default() {
    let result = StoreEvalMutResult::default();
    assert_eq!(result, StoreEvalMutResult::None);
}

#[test]
fn test_eval_mut_result_size() {
    let result = StoreEvalMutResult::default();
    assert_eq!(result.size(), 0u8.size());
}

#[test]
fn test_eval_mut_result_check() {
    let result = StoreEvalMutResult::default();
    let res = result.check();
    assert!(res.is_ok());
}

#[test]
fn test_eval_mut_result_serialize_json() {
    let result_a = StoreEvalMutResult::default();
    
    let res = result_a.to_json();
    assert!(res.is_ok());
    let json = res.unwrap();

    let res = StoreEvalMutResult::from_json(&json);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_eval_mut_result_serialize_bytes() {
    let result_a = StoreEvalMutResult::default();
    
    let res = result_a.to_bytes();
    assert!(res.is_ok());
    let bytes = res.unwrap();

    let res = StoreEvalMutResult::from_bytes(&bytes);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}

#[test]
fn test_eval_mut_result_serialize_hex() {
    let result_a = StoreEvalMutResult::default();
    
    let res = result_a.to_hex();
    assert!(res.is_ok());
    let hex = res.unwrap();

    let res = StoreEvalMutResult::from_hex(&hex);
    assert!(res.is_ok());
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);
}