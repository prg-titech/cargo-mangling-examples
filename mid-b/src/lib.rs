pub use url::Url;
pub fn consume(_u: Url) {}
pub fn consume_str(_s: &str) {}

// for ng2
pub fn port_or_default(s: &str) -> Result<Option<u16>, url::ParseError> {
    Url::parse(s).map(|u| u.port_or_known_default())
}
