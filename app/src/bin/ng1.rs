fn main() {
    // mid_a depends on url v1 and re-exports its Url type.
    // Therefore, mid_a::make() produces a value of type url::Url (v1).
    let u = mid_a::make();   
    
    // mid_b depends on url v2 and expects its own version of url::Url as input.
    // Although both are referred to as `url::Url` in source code,
    // Cargo’s name-mangling of dependencies means that these are two
    // distinct, incompatible types.
    // As a result, passing a v1 Url into a function expecting a v2 Url
    // produces a compile-time type error.
    mid_b::consume(u);

    // Compilation fails
    // $ cargo build --bin ng1
    //   Compiling app v0.1.0 (cargo-mangling/app)
    // error[E0308]: mismatched types
    //   --> src/bin/ng1.rs:12:20
    //     |
    // 12  |     mid_b::consume(u);       
    //     |     -------------- ^ expected `mid_b::Url`, found `mid_a::Url`
    //     |     |
    //     |     arguments to this function are incorrect
    //     |
    // note: two different versions of crate `url` are being used; two types coming from two different versions of the same crate are different types even if they look the same
    //   --> .cargo/registry/src/index.crates.io-1949cf8c6b5b557f/url-1.7.2/src/lib.rs:154:1
}
