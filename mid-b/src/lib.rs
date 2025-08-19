pub use url::Url;        // こちらは v2 由来
pub fn consume(_u: Url) {}
pub fn consume_str(_s: &str) {}

// ng2 用：文字列を受け取り、v2 の Url で「パース→再シリアライズ」が
// 元の文字列と等しいことを *期待*（= しばしば間違った期待）
pub fn consume_str_strict(s: &str) {
    let parsed = Url::parse(s).expect("mid_b: parse failed");
    assert_eq!(parsed.as_str(), s, "mid_b: canonicalization changed the string");
}

use std::collections::HashMap;
// 文字列キーの辞書を “正規化後のURL文字列” で引く。
// 正規化で文字列が変わるとヒットせず panic（= stringly-typed 境界の罠）
pub fn lookup_by_canonical(map: &HashMap<String, i32>, s: &str) -> i32 {
    let u = Url::parse(s).expect("mid_b: parse failed");
    let key = u.as_str().to_string();           // ← v2の正規化後文字列
    *map.get(&key).expect("mid_b: key not found after canonicalization")
}

// ★ NG4: base と相対パスを「文字列で」受け取り、単純連結 vs 正規化 join を比較
pub fn naive_join_expect_equal(base: &str, rel: &str) {
    // (悪い) ただの文字列連結
    let naive = format!("{base}{rel}");

    // (正しい) URL としての join（正規化・解決を行う）
    let joined = Url::parse(base)
        .expect("mid_b: base parse failed")
        .join(rel)
        .expect("mid_b: join failed");

    assert_eq!(
        naive,
        joined.as_str(),
        "mid_b: naive string concat != URL join (semantic mismatch)"
    );
}