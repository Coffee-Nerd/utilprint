# UtilPrint

[![Crates.io](https://img.shields.io/crates/v/utilprint.svg)](https://crates.io/crates/utilprint)
[![Documentation](https://docs.rs/utilprint/badge.svg)](https://docs.rs/utilprint)
[![License](https://img.shields.io/crates/l/utilprint.svg)](https://github.com/coffee-nerd/utilprint/blob/main/LICENSE)

UtilPrint is a minimalist Rust crate designed to simplify the process of adding color and Unicode characters, such as emojis, to your terminal output. 

It provides a fast and easy-to-use interface for enhancing your console applications with visually appealing elements.
This is how it looks in a few different terminals. The aesthetics will depend on your terminal settings and support.

<p align="center">
  <img src="https://github.com/Coffee-Nerd/utilprint/assets/126441228/78c3badc-7e6b-4409-909e-9964f8c17475" alt="240318_14h00m37s_screenshot" width="32%">
  <img src="https://github.com/Coffee-Nerd/utilprint/assets/126441228/b150c535-43fc-4427-8f7a-7a60418006a2" alt="240318_14h01m02s_screenshot" width="32%">
  <img src="https://github.com/Coffee-Nerd/utilprint/assets/126441228/c4b98697-3908-4895-ad0d-a38a2dfe8375" alt="240318_14h01m27s_screenshot" width="32%">
</p>

## Features

- **Easy to Use**: UtilPrint offers a straightforward API that makes it simple to integrate color and Unicode characters into your terminal output.
- **Minimalistic**: The crate is designed to be lightweight and unobtrusive, focusing on providing essential functionality without unnecessary complexity.
- **Fast**: UtilPrint is optimized for performance, ensuring that adding color and emojis to your terminal output doesn't slow down your application.
- **Unicode Support**: Easily incorporate Unicode characters, including emojis, into your terminal output to create more engaging and expressive console applications.

## Installation

Add `utilprint` to your `Cargo.toml`:

```toml
[dependencies]
utilprint = "0.1.0"
```

Usage
Here's a quick example of how to use UtilPrint to add color and an emoji to your terminal output:


```rust
use utilprint::utilprint;

fn main() {
    utilprint("@rHello, @bworld! #1F600").unwrap();
}
```
This will print "Hello, world!" with "Hello" in red, "world" in blue, and a smiley face emoji at the end.

If you're sure that your utilprint function will not fail, or if you're just writing a quick example, you can omit the .unwrap() for simplicity. Just keep in mind that if an error does occur, the program will fail silently without any indication of what went wrong.

**Color Codes**

UtilPrint uses simple color codes to specify colors:

```
@d: Black
@r: Red
@g: Green
@y: Yellow
@b: Blue
@m: Magenta
@c: Cyan
@w: White
@D: Gray (Bright Black)
@R: Bright Red
@G: Bright Green
@Y: Bright Yellow
@B: Bright Blue
@M: Bright Magenta
@C: Bright Cyan
@W: Bright White
@u: Reset color
For Xterm 256 colors, use @xNNN where NNN is the color number.

Unicode Characters
To add Unicode characters, use #NNNN where NNNN is the Unicode code. For example, #1F600 for a smiley face.
```

**Contributing**

Contributions are welcome! Please feel free to submit pull requests, report bugs, or suggest new features.

**License**

UtilPrint is licensed under the MIT License. Have fun using or modifying it!
