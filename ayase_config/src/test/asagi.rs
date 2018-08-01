use asagi::Value;
use test::utils::{assert, from_file};

#[test]
fn bibanon() {
    assert(from_file::<Value, _>("test/asagi/bibanon.json"));
}

#[test]
fn default() {
    assert(from_file::<Value, _>("test/asagi/default.json"));
}