use rstest::rstest;
use simplematch::{Options, SimpleMatchError, Wildcard};

#[rstest]
#[case::default(None, None, None, None, true)]
#[case::any_is_custom(Some(b'%'), None, None, None, true)]
#[case::any_is_default_any(Some(b'*'), None, None, None, true)]
#[case::any_is_default_one(Some(b'?'), None, None, None, false)]
#[case::any_is_default_escape(Some(b'\\'), None, None, None, false)]
#[case::any_is_default_negate(Some(b'!'), None, None, None, false)]
#[case::any_is_custom_one(Some(b'_'), Some(b'_'), None, None, false)]
#[case::any_is_custom_escape(Some(b'#'), None, Some(b'#'), None, false)]
#[case::any_is_custom_negate(Some(b'^'), None, None, Some(b'^'), false)]
#[case::one_is_custom(None, Some(b'_'), None, None, true)]
#[case::one_is_default_any(None, Some(b'*'), None, None, false)]
#[case::one_is_default_one(None, Some(b'?'), None, None, true)]
#[case::one_is_default_escape(None, Some(b'\\'), None, None, false)]
#[case::one_is_default_negate(None, Some(b'!'), None, None, false)]
#[case::one_is_custom_escape(None, Some(b'#'), Some(b'#'), None, false)]
#[case::one_is_custom_negate(None, Some(b'^'), None, Some(b'^'), false)]
#[case::escape_is_custom(None, None, Some(b'#'), None, true)]
#[case::escape_is_default_any(None, None, Some(b'*'), None, false)]
#[case::escape_is_default_one(None, None, Some(b'?'), None, false)]
#[case::escape_is_default_escape(None, None, Some(b'\\'), None, true)]
#[case::escape_is_default_negate(None, None, Some(b'!'), None, false)]
#[case::escape_is_custom_negate(None, None, Some(b'^'), Some(b'^'), false)]
#[case::negate_is_custom(None, None, None, Some(b'^'), true)]
#[case::negate_is_default_any(None, None, None, Some(b'*'), false)]
#[case::negate_is_default_one(None, None, None, Some(b'?'), false)]
#[case::negate_is_default_escape(None, None, None, Some(b'\\'), false)]
#[case::negate_is_default_negate(None, None, None, Some(b'!'), true)]
fn options_verify(
    #[case] any: Option<u8>,
    #[case] one: Option<u8>,
    #[case] escape: Option<u8>,
    #[case] negate: Option<u8>,
    #[case] expected: bool,
) {
    let mut options = Options::default();
    if let Some(any) = any {
        options.wildcard_any = any;
    }
    if let Some(one) = one {
        options.wildcard_one = one;
    }
    if let Some(escape) = escape {
        options.wildcard_escape = escape;
    }
    if let Some(negate) = negate {
        options.class_negate = negate;
    }

    if expected {
        options.verify().unwrap();
    } else {
        assert_eq!(
            options.verify(),
            Err(SimpleMatchError::DuplicateCharacterAssignment)
        );
    }
}

#[test]
fn options_case_insensitive() {
    let actual = Options::<u8>::default().case_insensitive(true);
    let mut expected = Options::default();
    expected.case_sensitive = false;

    assert_eq!(actual, expected);
}

#[test]
fn options_with_custom_wildcard_any() {
    let actual = Options::<u8>::default().wildcard_any_with(b'%');
    let mut expected = Options::default();
    expected.wildcard_any = b'%';
    assert_eq!(actual, expected);
}

#[test]
fn options_with_custom_wildcard_one() {
    let actual = Options::<u8>::default().wildcard_one_with(b'_');
    let mut expected = Options::default();
    expected.wildcard_one = b'_';
    assert_eq!(actual, expected);
}

#[test]
fn options_with_default_escape() {
    let actual = Options::<u8>::default().enable_escape(true);
    let mut expected = Options::default();
    expected.wildcard_escape = u8::DEFAULT_ESCAPE;
    expected.is_escape_enabled = true;
    assert_eq!(actual, expected);
}

#[test]
fn options_with_custom_escape() {
    let actual = Options::<u8>::default().enable_escape_with(b'#');
    let mut expected = Options::default();
    expected.wildcard_escape = b'#';
    expected.is_escape_enabled = true;
    assert_eq!(actual, expected);
}

#[test]
fn options_with_default_classes() {
    let actual = Options::<u8>::default().enable_classes(true);
    let mut expected = Options::default();
    expected.class_negate = u8::DEFAULT_CLASS_NEGATE;
    expected.is_classes_enabled = true;
    assert_eq!(actual, expected);
}

#[test]
fn options_with_custom_negate() {
    let actual = Options::<u8>::default().enable_classes_with(b'^');
    let mut expected = Options::default();
    expected.class_negate = b'^';
    expected.is_classes_enabled = true;
    assert_eq!(actual, expected);
}

#[test]
fn options_verified() {
    let options = Options::<u8>::default().verified().unwrap();
    assert_eq!(options, Options::default());
}
