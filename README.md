# rusty-monad

An implementation for Monad in Rust as a prototype.

This repository requires Rust nightly 1.50 or the later version.

This repository includes following things:

- Type classes emulation: `Functor`, `Applicative`, `Monad`
- Examples for data types: `Option`, `Result`, `Id`, `List` (underconstruction)
- Do notation emulation by macro

Type classes implementations are inspired by [this post](https://www.fpcomplete.com/blog/monads-gats-nightly-rust/). Thank you for the awesome post! Other implementations are by myself.
