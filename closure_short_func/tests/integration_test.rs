use closure_short_func::Cacher;

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a: u32| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}