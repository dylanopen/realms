# The Realms Book

This is a full guide to writing programs using the Realms engine and the Rust
Programming Language.

## Chapter 1 - Setup

### Project setup

To begin, create a new `cargo` project with the `cargo new` command and `cd`
into that directory.

``` bash
cargo new mygame
cd mygame
```

Next, run this command to add Realms to your project:

``` bash
cargo add realms
```

### Compilation and optimisations

We now want to compile our project to allow us to use Realms' functionality.  
First, we will enable **optimisations** to improve our proogram's performance.

Add the following at the *end* of your `Cargo.toml`:

``` toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

> This will build an optimised version of the `realms` library, but will not
> optimise your program as much (to allow for fast compile times).  
> It's a good balance between performance and compile time.

Now that optimisations are enabled, let's build our project!  
Run this command:

``` bash
cargo build
```

... and wait for it to compile.

Once it has compiled, you are ready to create your first window.

## Chapter 2 - The Window

### Coding

Copy this code into your `src/main.rs` file. We will explain it line-by-line in
a second.

``` rust
use realms::*;

fn main() {
    let mut w = Window::new("It's a window!", 800, 600);
    let fill = Rect::fill(&w, Color::rgb(31, 127, 31));

    while w.is_running() {
        w.new_frame();
        fill.draw(&mut w);
    }
}
```

### Output

This code produces the following window:

![An 800x600 window with title "It's a window!" and filled with a green background](docs/res/green_window.png)

### Explanation

Let's break down each line of the code.


``` rust
use realms::*;
```

- Import everything from the `realms` library and bring it into scope.
- In real applications, you should only import what you need. Avoid using
  wildcard imports.

``` rust
let mut w = Window::new("It's a window!", 800, 600);
```

- `Window::new` creates a new window and framebuffer.
- We create it with the dimensions `800x600` and title `"It's a window!"`.
- We store this window in the mutable `w` variable.

``` rust
let fill = Rect::fill(&w, Color::rgb(31, 127, 31));
```

- `Rect::fill` creates a rectangle filling the entire screen.
- We need to pass in a reference to the window (`&w`) as it needs to know the
  dimensions of the window.
- We pass it a color with the RGB value `(31, 127, 31)`, which is a dark green.
- We then store this node in the `fill` variable.

``` rust
while w.is_running() {
    /* ... */
}
```

- We loop indefinitely until the window is either closed by the user, or the
  `w.running` field is set to `false`.

``` rust
w.new_frame();
```

This does two main things:

1. Updates the `time` field (which contains things like the *delta time*).
2. Displays the buffer so nodes are shown on the screen.

> Note: It is important that you call this method at the **START** of each
> game loop iteration, or nodes may not display properly.

``` rust
fill.draw(&mut w);
```

- The `draw` method on nodes is used to display the draw method to the frame
  buffer.
- It will then be displayed on the screen when `w.new_frame` is next called.
- We need to pass in a mutable (changable) reference to the window, as it needs
  to update the window's buffer.

## Chapter 3 - The Rectangle

The `Rect` struct contains information for a single rectangle node.

It has a:

- Position
- Size
- Color

For detailed information about the `Rect` struct, please see
[https://docs.rs/realms/latest/realms/struct.Rect.html].

### Creating a rectangle

To create a rectangle, use the `new` method:

``` rust
let rect = Rect::new(
    (x, y),
    (width, height),
    color
);
```

For example:

``` rust
let rect = Rect::new(
    (32.0, 32.0),
    (64.0, 64.0),
    Color::rgb(255, 127, 31)
);
```

### Drawing a rectangle

Like every other drawable node (a struct which implements the `NodeDraw` trait),
we can draw a rectangle to the buffer like this...

``` 
rect.draw(&mut w);
```

... where `w` is the window variable.

> Note: Make sure you call the draw function *after* the `w.new_frame()` call.

## Chapter 4 - Other Nodes

Here is a list of the available nodes in Realms, as well as links to their
documentation:

- `Pixel` - [documentation](https://docs.rs/realms/latest/realms/struct.Pixel.html)
- `Rect` - [documentation](https://docs.rs/realms/latest/realms/struct.Rect.html)

### Pixel

A node representing a single pixel on a frame buffer.

Usage:

``` rust
Pixel::new(
    position: (f32, f32),
    color: Color
);
```

### Rect

A node representing a series of pixels in a rectangle shape, with a position,
size and color.

Usage:

``` rust
Rect::new(
    position: (f32, f32),
    size: (f32, f32),
    color: Color
);
```

### Rect::fill

Shorthand for creating a rectangle with position `(0, 0)` and with the same size
as the window.

Usage:

``` rust
Rect::fill(
    color: Color
);
```

## Chapter 5 - The Keyboard

### Realms events

All events and state checks return a `bool`.  
This means they can be used in a simple `if` statement for control flow.

Almost all events require a reference to the active `Window` as their first
argument.

### `key_down`

Returns `true` if the specified key is down, otherwise `false`.

Example:

``` rust
if key_down(&w, Key::W) {
    println!("moving player up!");
}
```

[View the documentation for this function](https://docs.rs/realms/latest/realms/fn.key_down.html)

### `key_just_pressed`

Returns `true` if the specified key was pressed this frame, otherwise `false`.

Example:

``` rust
if key_just_pressed(&w, Key::Space) {
    println!("activated powerup!");
}
```

[View the documentation for this function](https://docs.rs/realms/latest/realms/fn.key_just_pressed.html)

### `key_just_released`

Returns `true` if the specified key was released this frame, otherwise `false`.

Example:

``` rust
if key_just_released(&w, Key::W) {
    println!("falling back down again!");
}
```

[View the documentation for this function](https://docs.rs/realms/latest/realms/fn.key_just_released.html)

## Chapter 6 - The Mouse

### `mouse_down`

Returns `true` if the specified mouse button is held down, otherwise `false`.

Example:

``` rust
if mouse_down(&w, MouseButton::Left) {
    println!("dragging object");
}
```

[View the documentation for this function](https://docs.rs/realms/latest/realms/fn.mouse_down.html)

### `mouse_down_left`

Returns `true` if the left mouse button is held down, otherwise `false`.

Example:

``` rust
if mouse_down_left(&w) {
    println!("left mouse button is down");
}
```

[View the documentation for this function](https://docs.rs/realms/latest/realms/fn.mouse_down_left.html)

### `mouse_down_middle`

Returns `true` if the left mouse button is held down, otherwise `false`.

Example:

``` rust
if mouse_down_middle(&w) {
    println!("middle mouse button is down");
}
```

[View the documentation for this function](https://docs.rs/realms/latest/realms/fn.mouse_down_middle.html)

### `mouse_down_right`

Returns `true` if the left mouse button is held down, otherwise `false`.

Example:

``` rust
if mouse_down_right(&w) {
    println!("right mouse button is down");
}
```

[View the documentation for this function](https://docs.rs/realms/latest/realms/fn.mouse_down_right.html)

### `mouse_pos`

Returns the x and y coordinates of the mouse cursor, relative to the top-left
of the window, as an `(i32, i32)` tuple.

Example:

``` rust
println!("The mouse is at position {:?}", mouse_pos(&w));
```

[View the documentation for this function](https://docs.rs/realms/latest/realms/fn.mouse_pos.html)

