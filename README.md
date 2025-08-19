# Cargo Dependency Name Mangling Examples (with `url` crate)

This repository demonstrates how **Cargo resolves multiple versions of the same crate** (here: [`url`](https://crates.io/crates/url)) and how this can lead to **subtle incompatibilities**.  

When two different versions of the same crate are simultaneously required, Cargo’s resolver allows them to coexist by **name-mangling the crate identifiers internally**. From Cargo’s point of view, this is a successful dependency resolution.  
However, from the program’s perspective, the result can be surprising: the **same nominal type name** (e.g., `url::Url`) is actually two different types, one from `url v1`, the other from `url v2`.  

This repository builds a set of **minimal examples** to illustrate the difference between:
- **Compile-time errors** that arise when different types are directly mixed, and  
- **Runtime bugs** that appear when type incompatibilities are hidden behind string conversions or different semantic assumptions.

The purpose is to highlight how dependency resolution can affect program meaning, and why exposing dependency-owned types in public APIs is fragile.

---

## Project Structure

---
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
---

- `mid_a` depends on `url = "1"` and re-exports `url::Url`.  
- `mid_b` depends on `url = "2"` and re-exports `url::Url`, with additional helper APIs.  
- `app` imports both and provides multiple binaries (`src/bin/*.rs`) to demonstrate different scenarios.

---

## Scenarios

Below we describe the scenarios. Each can be built or run independently (`cargo build --bin ...`, `cargo run --bin ...`).  

They are grouped into **OK** (program builds and runs successfully) and **NG** (the program fails, either at compile time or at runtime).

---

### OK1 (disjoint usage)

Use `mid_a::Url` and `mid_b::Url` independently, never crossing them.  
→ Both versions of `url` coexist without issue.

---
~/cargo-mangling/app$ cargo run --bin ok1
   Compiling app v0.1.0 (/home/user/cargo-mangling/app)
    Finished `dev` profile [unoptimized + debuginfo]
     Running `target/debug/ok1`
u1 = https://example.com/
---

**Observation:** As long as the data flows are separated, multiple versions can safely coexist.

---

### OK2 (string bridge)

Convert `mid_a::Url` to a `String` and pass it across the boundary.  
→ Works, because standard library types like `String` are unaffected by crate versioning.

---bash
~/cargo-mangling/app$ cargo run --bin ok2
    Finished `dev` profile [unoptimized + debuginfo]
     Running `target/debug/ok2`
---

**Observation:** Stringly-typed bridges preserve compilation, but may hide semantic drift.

---

### OK3 (explicit type bridge)

Convert the string to `mid_b::Url` immediately via `Url::parse`.  
→ Safe, no hidden mismatch, since everything beyond the conversion uses only `url v2`’s type.

---bash
$ cargo run --bin ok3
    Finished `dev` profile [unoptimized + debuginfo]
     Running `target/debug/ok3`
ok3: bridged mid_a::Url -> mid_b::Url via string (safe)
---

**Observation:** Explicit conversion is the safest approach when bridging between versions.

---

### NG1 (compile-time type error)

Passing `mid_a::Url` (from `url v1`) into a function expecting `mid_b::Url` (from `url v2`).  
→ Different crate IDs → different types → **compile-time failure**.

---bash
~/cargo-mangling/app$ cargo build --bin ng1
error[E0308]: mismatched types
  --> src/bin/ng1.rs:3:20
   |
3  |     mid_b::consume(u);
   |                    ^ expected `mid_b::Url`, found `mid_a::Url`
---

**Observation:** This is the *best failure mode*: the incompatibility is caught at compile time.

---

### NG2 (string canonicalization mismatch)

Expecting `parsed.as_str() == original`, but `url v2` canonicalizes:  
- hostnames are lowercased,  
- default ports like `:80` are removed,  
- `%7E` is normalized to `~`.  

→ Build succeeds, but runtime assertion fails.

---bash
~/cargo-mangling/app$ cargo run --bin ng2
thread 'main' panicked at ...:
assertion `left == right` failed: mid_b: canonicalization changed the string
  left: "http://example.com/%7Euser"
 right: "http://EXAMPLE.com:80/%7Euser"
---

**Observation:** Stringly-typed bridges can compile, but semantic changes surface at runtime.

---

### NG3 (dictionary key mismatch)

Store a value under the **original string**, but look it up using the **canonicalized string** from `url v2`.  
→ Runtime panic: key not found.

---bash
~/cargo-mangling/app$ cargo run --bin ng3
thread 'main' panicked at ...:
mid_b: key not found after canonicalization
---

**Observation:** Differences in canonicalization can break equality assumptions silently.

---

### NG4 (naive join vs. URL join)

Compare naive string concatenation with `Url::join`.  
→ Runtime panic: semantic mismatch (`https://example.com/a/b/../c` vs `https://example.com/a/c`).

---bash
~/cargo-mangling/app$ cargo run --bin ng4
thread 'main' panicked at ...:
assertion `left == right` failed: mid_b: naive string concat != URL join
  left: "https://example.com/a/b/../c"
 right: "https://example.com/a/c"
---

**Observation:** Semantics of relative URL resolution cannot be captured by string concatenation.

---

## Key Lessons

- Cargo permits multiple major versions of the same crate to coexist.  
- From Cargo’s perspective this is a *successful resolution*, but from the program’s perspective it can cause breakage.  
- Two crates exposing the same nominal type name (`url::Url`) from different versions define **distinct types**.  
- Problems surface when these types (or their string representations) cross public boundaries.  

**Recommendations for library authors:**
- Do not re-export dependency-owned types in public APIs.  
- Use explicit conversion (bridge to one canonical version).  
- Avoid stringly-typed bridges unless semantics are guaranteed stable.

---

## References

- [Cargo Book – Registries](https://doc.rust-lang.org/cargo/reference/registries.html)  
- [Cargo Book – Resolver](https://doc.rust-lang.org/cargo/reference/resolver.html)  
- [WHATWG URL Standard – canonicalization rules](https://url.spec.whatwg.org/)  
