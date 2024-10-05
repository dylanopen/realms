# realms example 1 - a basic window

## Code

``` rust
use realms::*; // import everything from realms

fn main()
{
    let mut w: Window = Window::new("Welcome to Realms", 800, 450); // create window frame
    let fill: Rect = Rect::fill(&w, Color::rgb(91, 23, 127)); // create a rectangle that fills the screen

    while w.is_running() { // loop while window is open
        w.new_frame(); // initialise a new window frame (and draw to screen)
        fill.draw(window: &mut w); // draw fill node to clear the screen purple
    }
}
```

## Output

![An 800x450 window with title "Welcome to Realms" and filled with a purple background](../res/purple_window.png)

