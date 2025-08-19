fn main() {
    let u = mid_a::make();   // mid_a::Url ＝ url v1 由来
    mid_b::consume(u);       // mid_b::Url ＝ url v2 由来 → 型不一致で失敗
}
