fn main() {
    // mid_a は mid_a 内だけで完結
    let u1 = mid_a::make();
    println!("u1 = {}", u1);

    // mid_b は mid_b 由来の型を自前で作って使う
    let u2 = mid_b::Url::parse("https://b.example/").unwrap();
    mid_b::consume(u2);

    // ポイント：データがmid_aとmid_b由来のもので混ざらないところ。これなら問題は起こらない。
}
