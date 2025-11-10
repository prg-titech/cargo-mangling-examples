pub use url::Url;
pub fn make() -> Url {
    Url::parse("https://example.com/").unwrap()
}

// for ng2
pub fn port_or_default(s: &str) -> Result<Option<u16>, url::ParseError> {
    Url::parse(s).map(|u| u.port_or_known_default())
}

// for ng3
pub fn make_form_target() -> impl url::form_urlencoded::Target {
    String::new()
}