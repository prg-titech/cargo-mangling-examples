use std::collections::HashMap;

fn main() {
    // 正規化される代表例：
    // - ホスト小文字化: EXAMPLE.com → example.com
    // - 既定ポート除去: :80 → （消える）
    // - %エンコード正規化: %7E → ~
    let s = "http://EXAMPLE.com:80/%7Euser";

    // “元文字列”をキーに登録
    let mut m = HashMap::<String, i32>::new();
    m.insert(s.to_string(), 42);

    // v2 側の正規化後文字列で引く → キーが一致せず panic
    let _ = mid_b::lookup_by_canonical(&m, s);
}
