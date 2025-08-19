# Cargo Dependency Name Mangling Examples (with `url` crate)

When two distinct versions of the same crate are required simultaneously, Cargo resolves the conflict by **name-mangling crate identifiers** and assigning them disjoint namespaces. From the resolver’s perspective, this is a valid solution to version constraints.  

For client code, however, the result can be counterintuitive: two crates may expose items with the same nominal path (e.g., `url::Url`), yet these represent distinct types. Even when structurally identical, the compiler treats them as incompatible. Moreover, their associated functions may differ in semantic contracts—such as assumptions about valid inputs, encodings, or canonicalization—introducing subtle incompatibilities despite successful compilation.

This repository demonstrates how **Cargo’s multi-version resolution** (using [`url`](https://crates.io/crates/url) as an example) can lead to such issues.

---

## Project Structure

```
cargo-mangling/
├── mid-a/        # depends on url v1, re-exports url::Url
├── mid-b/        # depends on url v2, re-exports url::Url + extra helpers
└── app/
    └── src/
        └── bin/
            ├── ng1.rs  # compile-time error
            ├── ng2.rs  # runtime error (canonicalization)
            ├── ng3.rs  # runtime error (dictionary key mismatch)
            ├── ng4.rs  # runtime error (string concat vs URL join)
            ├── ok1.rs  # works (disjoint usage)
            ├── ok2.rs  # works (string bridge, safe)
            └── ok3.rs  # works (explicit conversion to v2 Url)
```

- `mid_a` depends on `url = "1"` and re-exports `url::Url`.  
- `mid_b` depends on `url = "2"` and re-exports `url::Url`, with additional helper APIs.  
- `app` imports both and provides multiple binaries (`src/bin/*.rs`) to demonstrate different scenarios.

## Background: `Url` Incompatibilities and Problems

Rust’s package manager Cargo allows multiple versions of the same crate to coexist in a single build.  
To avoid accidental mixing, Cargo applies **crate name mangling**: each version is assigned a distinct identity (e.g., `url v1` and `url v2` are treated as unrelated crates).  
While safe from the resolver’s perspective, this strategy introduces both **type-level** and **semantic-level** incompatibilities.

### Structural / Type Incompatibilities
- `mid_a::Url` (from `url v1`) and `mid_b::Url` (from `url v2`) are considered entirely different types.  
- Even if both define an identical `Url` struct, the compiler rejects any direct mixing.

### Logical / Semantic Incompatibilities
- Successive versions of `url` have changed canonicalization rules, such as:
  - Lowercasing hostnames (`EXAMPLE.com` → `example.com`)
  - Stripping default ports (`:80`)
  - Normalizing percent encodings (`%7E` → `~`)
  - Resolving relative paths (`../`) in joins
- As a result, invariants like `s == Url::parse(s).to_string()` may fail across versions.

### Problems Caused by Name Mangling
- **Over-conservatism:** Even when two `Url` types are structurally identical, Cargo treats them as incompatible once their major versions differ. The compiler provides no way to declare them equivalent.  
- **String-based workarounds and runtime pitfalls:** A common workaround is to pass values as `String` or `&str` across version boundaries. This compiles because standard library types are unversioned, but differing canonicalization rules can break equality checks or dictionary lookups. The resulting failures often appear as panics or assertion errors in unrelated code paths, with no mention of `Url`, making them difficult to diagnose.  
- **Dependency-driven breakage:** These issues can occur without any application changes, introduced solely through dependency resolution (e.g., in *diamond dependency* scenarios). In practice, **name mangling allows the package manager to alter a program’s behavior.**

## Initial Setup
```bash
cargo build --manifest-path mid-a/Cargo.toml \
  && cargo build --manifest-path mid-b/Cargo.toml
```

## Scenarios
All of the scenarios below are triggered under ***Cargo’s crate name mangling for the `url` package*** (i.e., `url v1` and `url v2` are treated as distinct crate identities).
This name mangling ensures that even if the crates export types or functions with the same nominal names, they remain incompatible across versions.

They are grouped into **OK** (program builds and runs successfully) and **NG** (the program fails, either at compile time or at runtime). Each can be built or run independently (`cargo build --bin ...`, `cargo run --bin ...`). 

| Scenario | Mechanism                        | Build | Runtime | Risk profile                | Note                                                                             |
| -------- | -------------------------------- | ----- | ------- | --------------------------- | -------------------------------------------------------------------------------- |
| **OK1**  | Disjoint usage                   | ✅     | ✅       | **Safe**                    | Independent data flows never cross.                                              |
| **OK2**  | String bridge                    | ✅     | ✅       | ⚠ **Hidden semantic incompatibility** | Compiles, but string-based exchange may misalign semantics later.                |
| **OK3**  | Explicit parse bridge            | ✅     | ✅       | ⚠ **Maintainability risk**  | Safe at runtime, but fragile: depends on stability of v1’s output ↔ v2’s parser. |
| **NG1**  | Type mismatch                    | ❌     | –       | ✅ **Compilation failure**          | Compile-time error prevents unsafe mixing.                                       |
| **NG2**  | String canonicalization mismatch | ✅     | ❌       | ❌ **Runtime failure**       | Canonicalization changes surface as failed assertions.                           |
| **NG3**  | Dictionary key mismatch          | ✅     | ❌       | ❌ **Runtime failure**       | Equality assumptions broken by canonicalization.                                 |
| **NG4**  | Naive concat vs. URL join        | ✅     | ❌       | ❌ **Runtime failure**       | String concat cannot capture URL join semantics.                                 |

```bash
cd app/
cargo run --bin ok1
cargo run --bin ok2
cargo run --bin ok3 
cargo build --bin ng1 
cargo build --bin ng2
cargo run --bin ng2
cargo build --bin ng3
cargo run --bin ng3
cargo build --bin ng4
cargo run --bin ng4
```

### OK1 (disjoint usage)

Use `mid_a::Url` and `mid_b::Url` independently, never crossing them.  
→ Both versions of `url` coexist without issue.

```bash
~/app$ cargo run --bin ok1
   Compiling app v0.1.0 (/home/user/cargo-mangling/app)
    Finished `dev` profile [unoptimized + debuginfo]
     Running `target/debug/ok1`
u1 = https://example.com/
```

**Observation:** As long as the data flows are separated, multiple versions can safely coexist.

### OK2 (string bridge)

Convert `mid_a::Url` to a `String` and pass it across the boundary.  
→ Works, because standard library types like `String` are unaffected by crate versioning.

```bash
~/app$ cargo run --bin ok2
    Finished `dev` profile [unoptimized + debuginfo]
     Running `target/debug/ok2`
```

**Observation:** Stringly-typed bridges preserve compilation, but may hide semantic incompatibility.

### OK3 (explicit type bridge)

Convert the string to `mid_b::Url` immediately via `Url::parse`.  
→ Safe, no hidden mismatch, since everything beyond the conversion uses only `url v2`’s type.

```bash
~/app$ cargo run --bin ok3
    Finished `dev` profile [unoptimized + debuginfo]
     Running `target/debug/ok3`
ok3: bridged mid_a::Url -> mid_b::Url via string (safe)
```

**Observation:** Explicit conversion establishes a clear type boundary and avoids mixed-type flows.

**⚠Note** However, this approach implicitly relies on the stability of `url v1`’s string output and `url v2`’s parser behavior.
Any change in canonicalization (lowercasing, default port removal, percent-decoding) could silently break this bridge.
Thus, while it appears safe at runtime, it carries hidden maintainability risks.

### NG1 (compile-time type error)

Passing `mid_a::Url` (from `url v1`) into a function expecting `mid_b::Url` (from `url v2`).  
→ Different crate IDs → different types → **compile-time failure**.

```bash
~/app$ cargo build --bin ng1
error[E0308]: mismatched types
  --> src/bin/ng1.rs:3:20
   |
3  |     mid_b::consume(u);
   |     -------------- ^ expected `mid_b::Url`, found `mid_a::Url`
   |     |
   |     arguments to this function are incorrect
   |
note: two different versions of crate `url` are being used; two types coming from two different versions of the same crate are different types even if they look the same
   --> /.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/url-1.7.2/src/lib.rs:154:1
    |
154 | pub struct Url {
    | ^^^^^^^^^^^^^^ this is the found type `mid_a::Url`
    |
   ::: /home/yudaitnb/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/url-2.5.4/src/lib.rs:227:1
    |
227 | pub struct Url {
    | ^^^^^^^^^^^^^^ this is the expected type `mid_b::Url`
    |
   ::: src/bin/ng1.rs:4:13
    |
4   |     let u = mid_a::make();   
    |             ----- one version of crate `url` used here, as a dependency of crate `mid_a`
...
12  |     mid_b::consume(u);
    |     ----- one version of crate `url` used here, as a dependency of crate `mid_b`
    = help: you can use `cargo tree` to explore your dependency tree
note: function defined here
   --> /home/yudaitnb/cargo-mangling/mid-b/src/lib.rs:2:8
    |
2   | pub fn consume(_u: Url) {}
    |        ^^^^^^^
```

**Observation:** This is the *best failure mode*: the incompatibility is caught at compile time. This error is reported by the rustc compiler as being caused by the version selection.

**Note:** However, this also means that the build will fail until `mid_a` is updated. For the `app` developer to use `mid_b`, which depends on version 2 of the `URL` package, they must push the `mid_a` developers to provide an update.

### NG2 (string canonicalization mismatch)

Expecting `parsed.as_str() == original`, but `url v2` canonicalizes:  
- hostnames are lowercased,  
- default ports like `:80` are removed,  
- `%7E` is normalized to `~`.  

→ Build succeeds, but runtime assertion fails.

```bash
~/app$ cargo run --bin ng2
thread 'main' panicked at ...:
assertion `left == right` failed: mid_b: canonicalization changed the string
  left: "http://example.com/%7Euser"
 right: "http://EXAMPLE.com:80/%7Euser"
```

**Observation:** Stringly-typed bridges can compile, but semantic changes surface at runtime.

### NG3 (dictionary key mismatch)

Store a value under the **original string**, but look it up using the **canonicalized string** from `url v2`.  
→ Runtime panic: key not found.

```bash
~/app$ cargo run --bin ng3
thread 'main' panicked at ...:
mid_b: key not found after canonicalization
```

**Observation:** Differences in canonicalization can break equality assumptions silently.

### NG4 (naive join vs. URL join)

Compare naive string concatenation with `Url::join`.  
→ Runtime panic: semantic mismatch (`https://example.com/a/b/../c` vs `https://example.com/a/c`).

```bash
~/app$ cargo run --bin ng4
thread 'main' panicked at ...:
assertion `left == right` failed: mid_b: naive string concat != URL join
  left: "https://example.com/a/b/../c"
 right: "https://example.com/a/c"
```

**Observation:** Semantics of relative URL resolution cannot be captured by string concatenation.

## References

- [HOW RUST SOLVED DEPENDENCY HELL](https://stephencoakley.com/2019/04/24/how-rust-solved-dependency-hell)
- [Cargo Book – Registries](https://doc.rust-lang.org/cargo/reference/registries.html)  
- [Cargo Book – Resolver](https://doc.rust-lang.org/cargo/reference/resolver.html)  
- [WHATWG URL Standard – canonicalization rules](https://url.spec.whatwg.org/)  
