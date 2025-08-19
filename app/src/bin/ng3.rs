use std::collections::HashMap;

fn main() {
    // Another case of semantic mismatch arises when URL strings are used
    // as dictionary (HashMap) keys. url v2 canonicalizes URLs according to
    // its own rules, such as:
    //   - Lowercasing hostnames: "EXAMPLE.com" → "example.com"
    //   - Removing default ports: ":80" → (omitted)
    //   - Normalizing percent-encoding: "%7E" → "~"
    //
    // As a result, two strings that appear equivalent to the user may be
    // treated as distinct keys if one side is canonicalized but the other
    // is not.
    let s = "http://EXAMPLE.com:80/%7Euser";

    // Insert the *original string* as the dictionary key.
    let mut m = HashMap::<String, i32>::new();
    m.insert(s.to_string(), 42);

    // Now attempt to look up the entry using the url v2–canonicalized form.
    // Although both the stored key and the query are of type `String`,
    // the canonicalization changes their surface representation,
    // causing the lookup to fail. The absence of the key triggers a panic.
    let _ = mid_b::lookup_by_canonical(&m, s);

    // Compilation succeeds
    // ~/cargo-mangling/app$ cargo build --bin ng3
    //    Compiling app v0.1.0 (/cargo-mangling/app)
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s

    // but get panic
    // ~/cargo-mangling/app$ cargo run --bin ng3
    //    Compiling app v0.1.0 (/cargo-mangling/app)
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
    //      Running `target/debug/ng3`

    // thread 'main' panicked at /cargo-mangling/mid-b/src/lib.rs:18:20:
    // mid_b: key not found after canonicalization
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
