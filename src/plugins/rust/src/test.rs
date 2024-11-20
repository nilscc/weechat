use crate::call_test;

#[test]
fn test_c_call() {
    let str = call_test();
    assert_eq!("test", str);
}
