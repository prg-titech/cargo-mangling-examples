fn main() {
    // A canonical example of semantic divergence between naive string
    // concatenation and library-supported URL resolution:
    //
    // - String concatenation:
    //     "https://example.com/a/b/" + "../c"
    //     → "https://example.com/a/b/../c"
    //       (the apparent relative segment `../` is left unresolved)
    //
    // - URL join (url v2):
    //     base.join("../c")
    //     → "https://example.com/a/c"
    //       (the `../` segment is interpreted and normalized away)
    //
    // While both results are of type `String` and superficially intended
    // to represent "the same location," their semantics differ. Any code
    // that assumes equality across these interpretations will fail at
    // runtime (here via an assertion).
    let base = "https://example.com/a/b/";
    let rel  = "../c";

    mid_b::naive_join_expect_equal(base, rel);

    // Compilation succeeds
    // ~/cargo-mangling/app$ cargo build --bin ng4
    //    Compiling app v0.1.0 (/cargo-mangling/app)
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s

    // But get panic
    // ~/cargo-mangling/app$ cargo run --bin ng4
    //    Compiling app v0.1.0 (/cargo-mangling/app)
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
    //      Running `target/debug/ng4`
    //
    // thread 'main' panicked at /cargo-mangling/mid-b/src/lib.rs:32:5:
    // assertion `left == right` failed: mid_b: naive string concat != URL join (semantic mismatch)
    //   left: "https://example.com/a/b/../c"
    //  right: "https://example.com/a/c"
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
