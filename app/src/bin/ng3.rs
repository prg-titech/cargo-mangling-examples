fn main() {
    // mid_a depends on url v1 and returns an opaque value that implements
    // *its* re-exported `url::form_urlencoded::Target` (from v1’s dependency graph).
    let t = mid_a::make_form_target();

    // mid_b depends on url v2 and requires a type implementing
    // *its* `url::form_urlencoded::Target` (from v2’s dependency graph).
    // Even though the paths look the same in source, the trait identities can differ
    // across versions due to Cargo’s dependency graph, which can trigger a type error.
    mid_b::consume_target(t);

    // $ cargo run --bin ng3
    // error[E0277]: the trait bound `impl url::form_urlencoded::Target: form_urlencoded::Target` is not satisfied
    //   --> src/bin/ng3.rs:7:27
    //    |
    //  7 |     mid_b::consume_target(t);
    //    |     --------------------- ^ the trait `form_urlencoded::Target` is not implemented for `impl url::form_urlencoded::Target`
    //    |     |
    //    |     required by a bound introduced by this call
    //    |
    // note: required by a bound in `consume_target`
    //   --> /home/yudaitnb/cargo-mangling-examples/mid-b/src/lib.rs:11:26
    //    |
    // 11 | pub fn consume_target<T: url::form_urlencoded::Target>(mut t: T) {
    //    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `consume_target`

    // For more information about this error, try `rustc --explain E0277`.
    // error: could not compile `app` (bin "ng3") due to 1 previous error
}