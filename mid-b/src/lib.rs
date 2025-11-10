pub use url::Url;
pub fn consume(_u: Url) {}
pub fn consume_str(_s: &str) {}

// for ng2
pub fn port_or_default(s: &str) -> Result<Option<u16>, url::ParseError> {
    Url::parse(s).map(|u| u.port_or_known_default())
}

// for ng3
pub fn consume_target<T: url::form_urlencoded::Target>(mut t: T) {
    let s: &mut String = url::form_urlencoded::Target::as_mut_string(&mut t);
    s.push_str("hello");
    let _finished = url::form_urlencoded::Target::finish(t);
}