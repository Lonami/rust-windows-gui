# minimal-windows-gui

*miwig: MInimal, WIndows-focused Graphical user-interface toolkit.*

A crate to build Graphical User Interfaces for Windows applications, using just WinAPI.

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

## Why another toolkit?

I like to avoid heavy dependencies on my projects when possible, and if those dependencies are
my own, it's even more justifiable to use them!

I know there are other good options out there ([A 2021 Survey of Rust GUI Libraries][other-ui]
is a great summary), even Windows-focused alternatives such as [Native Windows GUI][nwg], but
that's like, not my own! And I can't prevent NWG from growing more than I'd want my own to.

So, this is my own personal take on this thing. It's very minimal. You can expect a certain
level of abstraction, but don't expect a full-blown DSL or fancy derive macros. If you're doing
anything half-serious, you may want to consider a different toolkit.

[winapi]: https://docs.microsoft.com/en-us/windows/win32/apiindex/windows-api-list
[winapi-tut]: http://winprog.org/tutorial/
[using-res]: http://winprog.org/tutorial/resources.html
[embed-resource]: https://crates.io/crates/embed_resource
[winres]: https://crates.io/crates/winres
[about-rc]: https://docs.microsoft.com/en-us/windows/win32/menurc/resource-compiler
[other-ui]: https://www.boringcactus.com/2021/10/24/2021-survey-of-rust-gui-libraries.html#native-windows-gui
[nwg]: https://gabdube.github.io/native-windows-gui/
