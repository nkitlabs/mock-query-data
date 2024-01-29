use test_query_data::{add, http::minus};

#[test]
fn plus_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn minus_works() {
    let result = minus(2, 2);
    assert_eq!(result, 0);
}
