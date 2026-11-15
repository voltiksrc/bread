# bread

bread is a minimal floating X11 window manager written in Rust using [x11rb](https://github.com/psychon/x11rb).

it's a small project I'm working on to learn more about X11, window management, and Rust. the goal is to keep it lightweight, simple, and customizable without adding a bunch of unnecessary features.

## Features

- floating window management
- click-to-focus
- focused and unfocused window borders
- basic window stacking
- configurable border widths (currently hardcoded)

## Building

You'll need Rust and Cargo installed.

```sh
git clone https://github.com/voltiksrc/bread
cd bread
cargo build --release
```

The executable will be in `target/release/bread`.

## Development

bread is still in early development and isn't ready to be used as a daily window manager.

I'm planning to add window dragging, resizing, keyboard shortcuts, and a custom configuration format.

For now, it's mainly a learning project, so expect bugs and missing features.

## License

Not specified yet.
