use arbitrary::Arbitrary;

#[derive(Debug, Arbitrary)]
pub struct FuzzOptions {
    pub use_ranges: bool,
    pub use_other_negate: bool,
    pub use_other_wildcard_any: bool,
    pub use_other_wildcard_one: bool,
    pub case_sensitive: bool,
    pub enable_escape: bool,
    pub use_other_wildcard_escape: bool,
}
