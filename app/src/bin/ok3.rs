fn main() {
    // Step 1: Obtain a Url from mid_a (backed by url v1).
    // We immediately convert it into a string, breaking the strong type link.
    let s = mid_a::make().to_string();

    // Step 2: Re-parse the string into the v2 Url type expected by mid_b.
    // This establishes a clean type boundary: from here onward, the value
    // is owned entirely by the v2 world.
    let u2 = mid_b::Url::parse(&s).expect("ok3: parse failed");

    // Step 3: Pass the v2 Url into mid_b as usual.
    mid_b::consume(u2);

    println!("ok3: bridged mid_a::Url -> mid_b::Url via string (safe)");

    // ⚠ Note:
    // Although this "string bridge" prevents compile-time type errors
    // and runs correctly, it implicitly relies on url v1's `to_string()`
    // output being stably accepted by url v2's `parse()`.
    // Any change in canonicalization rules or textual conventions
    // (e.g., case-folding, default ports, percent-encoding)
    // could silently break this bridge.
    // Therefore, the approach carries hidden maintainability risks,
    // even though it appears safe at runtime.
}
