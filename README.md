# winapi-app-windows

A crate to build applications' windows in Windows using WinAPI. This would be less confusing if
the operating system was called something else. Better name ideas are welcome.

The primary goal is to build a better abstraction for Rust to deal with the [Windows API][winapi],
and the development is tailored by [theForger's Win32 API Programming Tutorial][winapi-tut] which
is a great source to get into the API. The examples from that resource are ported to Rust using
this crate under the `examples/` folder, which can be ran with `cargo`:

```sh
# for small single-file examples
cargo run --example test

# for larger examples using resources
cargo run --package menu_one
```

For [Using Resources][using-res], a separate crate such as [embed-resource] or [winres] may be
used for commodity (essentially emitting the correct `cargo:rustc-link-lib` value in `build.rs`).
You may also [read about `.rc` files][about-rc] to understand what they are and how they work.

[winapi]: https://docs.microsoft.com/en-us/windows/win32/apiindex/windows-api-list
[winapi-tut]: http://winprog.org/tutorial/
[using-res]: http://winprog.org/tutorial/resources.html
[embed-resource]: https://crates.io/crates/embed_resource
[winres]: https://crates.io/crates/winres
[about-rc]: https://docs.microsoft.com/en-us/windows/win32/menurc/resource-compiler
