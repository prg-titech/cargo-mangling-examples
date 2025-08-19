fn main() {
    let u = mid_a::make();
    let s = u.as_str();     // または u.to_string()
    mid_b::consume_str(s);  // 文字列APIならバージョン差は表に出ない

    // ポイント：データがmid_aとmid_b由来のもので混ざっているが、primitive値を使っているだけなので問題は起こらない。
}
