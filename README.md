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

## Ported theForger's examples

To run a `file.rs`-based example, use:

```sh
cargo run --example file
```

To run a `folder/`-based example, use:

```sh
cargo run --manifest-path examples/folder/Cargo.toml
```

To run all of them:

```sh
for f in examples/*; do test -f "$f" && { cargo run --example $(basename "$f" .rs); } || { cargo run --manifest-path $f/Cargo.toml; }; done
```

* Basics
  * Getting Started: `intro.rs`. Displays a message box and exits.
  * A Simple Window: `simple_window.rs`. A blank window with default behaviour.
  * Handling Messages: `window_click.rs`. A window that reacts to click events.
  * The Message Loop: No full program examples in this section.
  * Using Resources:  No full program examples in this section.
  * Menus and Icons: `menu_one/`, `menu_two.rs`. A window with toolbar actions, using resources and created programatically, respectively.
  * Dialog Boxes: `dlg_one/`. A window with a toolbar action to open a custom About dialog.
  * Modeless Dialogs: `dlg_two/`. A window with a dialog pre-opened that cannot be closed.
  * Standard Controls: `ctl_one/`. A complex window layout with text inputs, scrollable lists, and buttons.
  * Dialog FAQ: `dlg_three/`. A window with a custom background color.
* Creating a simple application
  * Creating controls at runtime. `app_one.rs`. A window with a text area with scrollbars in both directions and a resize grip.
  * Files and the common dialogs. `app_two.rs`. Builds on top of `app_one` and adds a window with a toolbar to open the standard *Open File* and *Save As* dialogs.
  * Tool and Status bars. `app_three.rs`. Builds on top of `app_two` and adds a status bar and icon bar as an alternative to the toolbar.
  * Multiple Document Interface. Not ported yet. Builds on top of `app_three` and adds nested windows that are contained within the main window.
* Graphics Device Interface
  * Bitmaps and Device Contexts. `bmp_one/`. A window that loads and displays a bitmap.
  * Transparency. `bmp_two.rs`. A window that loads and displays a bitmap with different masks applied.
  * Timers and Animation. `anim_one.rs`. A window that loads a bitmap and uses a timer to animate it.
  * Text, Fonts and Colours. Not ported yet. A window that uses a custom font in its text area.

[winapi]: https://docs.microsoft.com/en-us/windows/win32/apiindex/windows-api-list
[winapi-tut]: http://winprog.org/tutorial/
[using-res]: http://winprog.org/tutorial/resources.html
[embed-resource]: https://crates.io/crates/embed_resource
[winres]: https://crates.io/crates/winres
[about-rc]: https://docs.microsoft.com/en-us/windows/win32/menurc/resource-compiler
[other-ui]: https://www.boringcactus.com/2021/10/24/2021-survey-of-rust-gui-libraries.html#native-windows-gui
[nwg]: https://gabdube.github.io/native-windows-gui/
