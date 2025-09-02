#[test]
fn test_wildcard() {
    let bytes = "a*?b".as_bytes();
    assert!(wildcard::Wildcard::new(bytes)
        .unwrap()
        .is_match("aab".as_bytes()))
}

#[test]
fn test_wildcard_2() {
    let bytes = "a*?b".as_bytes();
    assert!(wildcard::Wildcard::new(bytes)
        .unwrap()
        .is_match(format!("{}b", "a".repeat(100)).as_bytes()))
}
