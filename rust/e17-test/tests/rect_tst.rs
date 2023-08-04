use e17test;

#[test]
fn integration_test_1() {
    let r = e17test::add(1, 5);
    assert_eq!(6, r);
}