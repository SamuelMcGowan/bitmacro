# Bitmacro

A bitfield crate implemented with declarative macros. [^1]

[^1]: And the [`paste`](https://github.com/dtolnay/paste) crate.

Inspired by [`bilge`](https://github.com/hecatia-elegua/bilge), `bitmacro` supports arbitrary-bitwidth integers from the [`arbitrary-int`](https://github.com/danlehmann/arbitrary-int) crate, `bool`s and other bitfields as fields. Bitfields can also have arbitrary bitwidth, which is useful when using a bitfield as a field in another. Additionally enums can be used as bitfields too, where each bit pattern is represented by a variant.

Work in progress - no documentation yet! In the meantime, you can reference [this simple example](examples/simple.rs)/
