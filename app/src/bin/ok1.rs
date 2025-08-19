fn main() {
    // mid_a is used only within its own scope.
    let u1 = mid_a::make();
    println!("u1 = {}", u1);

    // mid_b constructs and consumes its own value,
    // fully within the v2 world.
    let u2 = mid_b::Url::parse("https://b.example/").unwrap();
    mid_b::consume(u2);

    // Key point: values originating from mid_a and mid_b
    // never cross dataflow boundaries. With no intermixing
    // across crate versions, no incompatibility can arise.
}
