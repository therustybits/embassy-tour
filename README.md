# Embassy Tour

This is the code for the [YouTube video](https://youtu.be/pDd5mXBF4tY), where we take a brief tour of some of the crates within the `embassy` framework. The code for the end of each chapter is in its respective directory. Nothing groundbreaking here: just a resource for anyone that wants to play around with the code from the video :)

## Prerequisites

As of `embassy-executor` v0.7, you'll need Rust v1.83 or higher: be sure to run `rustup update` to avoid wonky `Waker`-related errors.. For those just getting started with Rust in embedded systems:
- [Getting setup for embedded development in Rust](https://youtu.be/TOAynddiu5M)
- [HALs & PACs : exploring the embedded Rust ecosystem](https://youtu.be/A9wvA_S6m7Y)
- (Recommended) [The Rust Book](https://doc.rust-lang.org/book/)

## Further Research

- [zero-to-async](https://youtu.be/wni5h5vIPhU) : building an async runtime from scratch
- [embassy-examples](https://github.com/embassy-rs/embassy/tree/main/examples) : lots of code demonstrating how to use different MCU peripherals & `embassy` crates
- [Dario Nieuwenhuis' 2024 RustNL talk introducing `embassy`](https://youtu.be/H7NtzyP9q8E)
- [embassy Matrix chat](https://matrix.to/#/#embassy-rs:matrix.org) : lots of friendly & very knowledgable people hang out here