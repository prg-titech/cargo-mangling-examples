fn main() {
    // ベース+相対の典型ケース：
    // - 文字列連結: "https://example.com/a/b/" + "../c"
    //     → "https://example.com/a/b/../c"  （見かけの相対セグメントが残る）
    // - URL join:   正規化で "../" を解決
    //     → "https://example.com/a/c"
    //
    // これを「同じ」と期待するのは誤りで、実行時に assert が落ちる。
    let base = "https://example.com/a/b/";
    let rel  = "../c";

    mid_b::naive_join_expect_equal(base, rel);
}
