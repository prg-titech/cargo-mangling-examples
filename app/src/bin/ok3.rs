fn main() {
    // v1 由来の Url を一旦文字列に
    let s = mid_a::make().to_string();

    // 受け取り側（v2）の型にパースし直す
    let u2 = mid_b::Url::parse(&s).expect("ok3: parse failed");

    // 以降は "v2のUrl" として扱うので混線しない
    mid_b::consume(u2);

    println!("ok3: bridged mid_a::Url -> mid_b::Url via string (safe)");
}
