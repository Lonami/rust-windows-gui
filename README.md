# winapi-app-windows

A crate to build applications' windows in Windows using WinAPI. This would be less confusing if
the operating system was called something else. Better name ideas are welcome.

The primary goal is to build a better abstraction for Rust to deal with the [Windows API][0],
and the development is tailored by [theForger's Win32 API Programming Tutorial][1] which is a
great source to get into the API. The examples from that resource are ported to Rust using this
crate under the `examples/` folder, which can be ran with `cargo`:

```sh
cargo run --example test
```

[0]: https://docs.microsoft.com/en-us/windows/win32/apiindex/windows-api-list
[1]: http://winprog.org/tutorial/
