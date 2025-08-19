pub use url::Url;        // 型を公開
pub fn make() -> Url {
    Url::parse("https://example.com/").unwrap()
}
