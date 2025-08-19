fn main() {
    let u = mid_a::make();

    // Extract only a primitive view of the value.
    // (as_str or to_string both erase version-specific structure)
    let s = u.as_str();

    // mid_b consumes the primitive string.
    mid_b::consume_str(s);

    // Key point: data originating from mid_a and mid_b
    // *is* intermixed here, but only through primitive values
    // (strings). Since primitives are version-agnostic,
    // no semantic mismatch is exposed.
}
