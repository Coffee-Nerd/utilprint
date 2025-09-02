# UtilPrint

[![Crates.io](https://img.shields.io/crates/v/utilprint.svg)](https://crates.io/crates/utilprint)
[![Documentation](https://docs.rs/utilprint/badge.svg)](https://docs.rs/utilprint)
[![License](https://img.shields.io/crates/l/utilprint.svg)](https://github.com/coffee-nerd/utilprint/blob/main/LICENSE)

UtilPrint is a minimalist Rust crate designed to simplify the process of adding color and Unicode characters, such as emojis, to your terminal output.

It provides a fast and easy-to-use interface for enhancing your console applications with visually appealing elements.
This is how it looks in a few different terminals. The aesthetics will depend on your terminal settings and support.

![240727_00h54m57s_screenshot](https://github.com/user-attachments/assets/a9704b4b-9a14-44d8-a676-152171b9a092)

## Features

- **Easy to Use**: UtilPrint offers a straightforward API that makes it simple to integrate color and Unicode characters into your terminal output.
- **Minimalistic**: The crate is designed to be lightweight and unobtrusive, focusing on providing essential functionality without unnecessary complexity.
- **Fast**: UtilPrint is optimized for performance, ensuring that adding color and emojis to your terminal output doesn't slow down your application.
- **Unicode Support**: Easily incorporate Unicode characters, including emojis, into your terminal output to create more engaging and expressive console applications.

## Installation

Add `utilprint` to your `Cargo.toml`:

```toml
[dependencies]
utilprint = "0.1.5"
```

Usage
Here's a quick example of how to use UtilPrint to add color and an emoji to your terminal output:

```rust
use utilprint::utilprint;

fn main() {
    utilprint("@x226He@x190ll@x155o @x119W@x120or@x084l@x085d, @x050t@x051his @x045is@x075 U@x069t@x105il@x099P@x135ri@x165nt@x201, let@x200's m@x205ake t@x204he@x210 w@x209or@x215ld co@x220lorf@x226ul!@w Here is a #2615, @ua Euro #20AC @Rs@ry@Rm@rb@Ro@rl@u, @Mand p@mu@Mr@mp@Ml@me @Mt@me@Mx@mt");
}
```

This will print "Hello World, this is UtilPrint, let's make the world Colorful!"

In rainbow colors, and then it includes more text in different colors, and some unicode characters, like an emoji.
Here is a ☕, a Euro € symbol, and purple text

That is a complex example, but you may think that isn't that easy to read, and you may want to use just one color, or a specific set of colors.
For this I have implemented many different cool methods. 

```rust
utilprint("I wonder what cool things we are going to make with UtilPrint?!".pastelbow());
```
![image](https://github.com/user-attachments/assets/4c7159fd-1c73-43f2-8fbe-93856bc283de)

As you can see, this is now printing in our custom colorset, called pastelbow!

```rust
utilprint("I really like rainbows, they are beautiful and awesome!".rainbow());
```
![image](https://github.com/user-attachments/assets/07efc9c4-4c39-4160-837d-de866642dcaa)

This would also create a more traditional rainbow colored text!

```rust
utilprint("This should be red!".red());
```
![image](https://github.com/user-attachments/assets/da242adc-15e0-4ccb-bdd3-c5c4fbe30506)

We can also use very simple coloration, with simply using the `.red()` method.

```rust
utilprint("Random XTERM gradient colors!".random_xterm_gradient());
```

This creates a smooth gradient using XTERM 256 colors with random start and end colors that change each time you run it!

```rust
utilprint("Random true color gradients are amazing!".random_truecolor_gradient());
```

This creates a smooth gradient using 24-bit true colors (RGB) with random start and end colors that change each time you run it!

And of course we love unicode, and emojis! 
You can see a nice list of unicode symbols here... 

https://www.compart.com/en/unicode/category/So

Currently we support unicode in the form of #0000, where the four zeroes can be any combination of numbers and letters...

```rust
utilprint("I love to drink #2615 every single day, because it makes me #263A #2705");
```
![image](https://github.com/user-attachments/assets/e92a9b13-a1f2-480f-8d98-64d0c89aaa38)

Here you can see we have our coffee, our smiley face, and our checkmark! 

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

This project uses an MIT license. Feel free to use this, but credit me!

**KNOWN BUGS** 

Using a method like `.rainbow()`, or any non single color method like `.red()` will result in your emojis being lost, and the unicode
not being consumed. This is in the works to be fixed.

UtilPrint is licensed under the MIT License. Have fun using or modifying it! If you want to credit me, that would be great!
