fn main() {
    // Trigger the [`approx_constant`][lint] lint from `clippy` on purpose.
    // [lint]: https://rust-lang.github.io/rust-clippy/master/#approx_constant
    println!("{}", 3.14);
}
