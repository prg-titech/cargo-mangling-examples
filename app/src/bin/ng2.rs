fn main() {
    // At first glance, this appears to be a single, unambiguous URL string.
    // However, url v2 applies canonicalization rules that differ from how
    // a raw string might be interpreted. In particular:
    //   - Hostnames are lowercased (e.g., "EXAMPLE.com" → "example.com")
    //   - Default ports (e.g., ":80" for HTTP) are removed
    //   - Percent-encoded sequences may be normalized (e.g., "%7E" → "~")
    //
    // As a consequence, the invariant
    //     s == Url::parse(s).to_string()
    // does not always hold under v2 semantics.
    //
    // Here we deliberately choose a representative counterexample
    // to demonstrate the semantic mismatch.
    let s = "http://EXAMPLE.com:80/%7Euser";

    // (Optionally, one might imagine this string being produced by mid_a,
    //   e.g., `let s = mid_a::make().to_string();`
    //   However, url v1 may already normalize certain components,
    //   so we use a raw literal here to ensure reproducibility.)
    
    // mid_b expects a canonicalized form and asserts equality against it.
    // Although compilation succeeds (since both sides are `String`),
    // the semantic discrepancy triggers a runtime assertion failure.
    mid_b::consume_str_strict(s);

    // Compilation succeeds
    // ~/cargo-mangling/app$ cargo build --bin ng2
    //      Compiling app v0.1.0 (/cargo-mangling/app)
    //       Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s

    // but assertion failed
    // ~/cargo-mangling/app$ cargo run --bin ng2
    //   Compiling app v0.1.0 (/cargo-mangling/app)
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
    //     Running `target/debug/ng2`
    //
    // thread 'main' panicked at /cargo-mangling/mid-b/src/lib.rs:9:5:
    // assertion `left == right` failed: mid_b: canonicalization changed the string
    //   left: "http://example.com/%7Euser"
    //  right: "http://EXAMPLE.com:80/%7Euser"
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
