fn main() {
    // Cross-version semantic mismatch with only primitive return types.
    //
    // Context:
    // - `mid_a` depends on `url v1`
    // - `mid_b` depends on `url v2`
    // Both expose the same helper `port_or_default(s) -> Result<Option<u16>, ParseError>`.
    //
    // Why this example?
    // - The function `Url::port_or_known_default()` returns the *explicit* port if present,
    //   otherwise it may return the scheme’s *default* port if that scheme is considered
    //   “known/special” by the implementation.
    // - Between `url v1` and `url v2`, the set of schemes with known defaults diverged
    //   to match the WHATWG “special schemes” list more closely.
    // - Historically, `gopher` has a default port of 70 (IANA / legacy behavior).
    //   `url v1` commonly reports this as `Some(70)`, while `url v2`—where `gopher`
    //   is *not* treated as a special scheme—returns `None` when the port is not
    //   explicitly present.
    //
    // Consequence:
    // - The **type** at the API boundary is identical in both versions: `Option<u16>`.
    // - The **semantics** differ: for the same input, v1 may report `Some(70)`,
    //   v2 may report `None`.
    // - This compiles cleanly (nothing but standard library types cross the boundary),
    //   but leads to a **runtime assertion failure** when the results are compared.
    let s = "gopher://example.com/";

    // Evaluate under v1 (via mid_a) and v2 (via mid_b).
    let p1 = mid_a::port_or_default(s).expect("mid_a(v1): parse failed");
    let p2 = mid_b::port_or_default(s).expect("mid_b(v2): parse failed");

    // If v1 treats `gopher` as having a known default port (70) but v2 does not,
    // this equality check will fail at runtime:
    //   v1: Some(70)
    //   v2: None
    assert!(
        p1 == p2,
        "NG2: port_or_default mismatch:\n  v1: {:?}\n  v2: {:?}\n  src: {}",
        p1, p2, s
    );

    // thread 'main' panicked at src/bin/ng2.rs:9:5:
    // NG2: port_or_default mismatch:
    //   v1: Some(70)
    //   v2: None
    //   src: gopher://example.com/

    println!("ng2: no mismatch on this pair (try other inputs).");
}
