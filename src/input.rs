use std::path::PathBuf;

/// This enum defines the different window events.
/// You should match against these when looping over `w.events()`.
///
/// ## Example usage:
///
/// ``` rust
/// while w.is_running() {
///     w.new_frame();
///     for event in window.events() {
///         match event {
///             Event::KeyDown(Key::Q) => w.close();
///             _ => {}
///         }
///     }
/// }
/// ```
#[derive(Debug)]
pub enum Event {
    
    /// The user has closed the window.
    Close,

    /// The `Key` $1 was pressed *down* by the user.
    KeyDown(Key),

    /// The `Key` $1 was *released* by the user.
    KeyUp(Key),

    /// The `Key` $1 was *repeated* (and was already held down).
    KeyRepeat(Key),

    /// Something happened to the window that means it needs to be
    /// refreshed. This is called upon many events, such as moving or
    /// resizing the window.
    RefreshWindow,

    /// The window was resized by the user, changing the `width` and `height`
    /// of the opengl viewport.
    /// This event is partially handled by Realms: the viewport size is updated
    /// to the size of the window.
    ///
    /// ## Parameters:
    /// - $1: the new **width** of the window.
    /// - $2: the new **height** of the window.
    ResizeWindow(i32, i32),


    /// The window was moved/relocated.
    /// This event is rarely used. However, it can be used in some situations
    /// to, for example, give objects a fixed/absolute position on the screen.
    ///
    /// ## Parameters:
    /// - $1: the new **x position** of the window.
    /// - $2: the new **y position** of the window.
    MoveWindow(i32, i32),

    /// A character was typed on the window.
    /// This is useful for text editing, as it returns the actual character
    /// that was typed on the keyboard (e.g. if the user pressed shift+1, a
    /// `TypeChar('!')` event is created).
    ///
    /// ## Parameters:
    /// - $1: the character typed as a `char` object.
    TypeChar(char),

    /// The user scrolled the mouse wheel / touchpad by the given distance.
    ///
    /// ## Parameters:
    /// - $1: the **horizontal (x) scroll distance** (this is rarely used).
    /// - $2: the **vertical (y) scroll distance**.
    Scroll(f64, f64),

    /// The user dragged and dropped a file / multiple files into the window.
    /// You can find where they dropped the file using the current mouse pos.
    ///
    /// ## Parameters:
    /// - $1: a vector of `PathBuf` objects pointing to the file dropped.
    DropFiles(Vec<PathBuf>),

    /// The user moved their mouse cursor to the new position.
    /// The coordinates are **relative to the window** position: (0.0, 0.0) is
    /// the **top left** of the window.
    /// You can keep track of the mouse pos by storing the x and y coords each
    /// time this event is present.
    ///
    /// ## Parameters:
    /// - $1: the `x coordinate` of the mouse cursor
    /// - $2: the `y coordinate` of the mouse cursor
    MoveMouse(f64, f64),

    /// The window was focused.
    /// This could be because the user alt-tabbed to the window or clicked on
    /// it.
    Focus,

    /// The window was brought out of focus.
    /// This could be because the user alt-tabbed away from the window or
    /// clicked on a different window.
    Unfocus,

    /// The mouse was hovered over the window (while previously being outside
    /// the window).
    /// This event is created when the user moves their cursor from outside our
    /// window to inside it.
    CursorEnter,

    /// The mouse was moved outside of the window (while previously being
    /// inside the window).
    /// This event is created when the user moves their cursor from inside our
    /// window to outside it.
    CursorExit,

    /// The specified `MouseButton` was pressed **down**.
    /// This event is called when the user pressed down one of the mouse
    /// buttons.
    /// 
    /// ## Parameters:
    /// - $1: the MouseButton that was pressed. Can be `Left`, `Right` or
    ///   `Middle`. See the `MouseButton` enum for more info.
    MouseDown(MouseButton),

    /// The specified `MouseButton` was **released**
    /// This event is called when the user stops pressing down one of the mouse
    /// buttons.
    /// 
    /// ## Parameters:
    /// - $1: the MouseButton that was released. Can be `Left`, `Right`,
    ///   `Middle` or `Other`. See the `MouseButton` enum for more info.
    MouseUp(MouseButton),

    /// An event that is supported by glfw but not currently supported by
    /// Realms.
    /// 
    /// > Important note: if, in the future, Realms adds a feature you previously
    /// > used the `Other` variant for, it will no longer be matched to `Other`.
    /// > If this event is not being handled after a Realms upgrade, check if
    /// > it was introduced natively to Realms.
    ///
    /// ## Parameters:
    /// - $1: the GLFW window event that is not supported by Realms.
    Other(glfw::WindowEvent),
}

impl Event {
    
    /// Convert a `glfw::WindowEvent` into a `realms::Event`.
    /// You don't need to call this method: it is public only so that
    /// `window.events()` can convert the glfw events into Realms events.
    pub fn from_glfw(glfw_event: glfw::WindowEvent) -> Event {
        match glfw_event {
            glfw::WindowEvent::Close
                => Event::Close,

            glfw::WindowEvent::Refresh
                => Event::RefreshWindow,

            glfw::WindowEvent::FramebufferSize(width, height)
                => Event::ResizeWindow(width, height),

            glfw::WindowEvent::Pos(x, y)
                => Event::MoveWindow(x, y),

            glfw::WindowEvent::Char(ch)
                => Event::TypeChar(ch),

            glfw::WindowEvent::Scroll(scroll_x, scroll_y)
                => Event::Scroll(scroll_x, scroll_y),

            glfw::WindowEvent::FileDrop(file_bufs)
                => Event::DropFiles(file_bufs),

            glfw::WindowEvent::CursorPos(x, y)
                => Event::MoveMouse(x, y),

            glfw::WindowEvent::Focus(is_focused) => match is_focused {
                true => Event::Focus,
                false => Event::Unfocus,
            }

            glfw::WindowEvent::CursorEnter(just_entered) => match just_entered {
                true => Event::CursorEnter,
                false => Event::CursorExit,
            }

            glfw::WindowEvent::MouseButton(glfw_mouse_button, action, _) => {
                let mouse_button = MouseButton::from_glfw(glfw_mouse_button);
                match action {
                    glfw::Action::Press => Event::MouseDown(mouse_button),
                    glfw::Action::Release => Event::MouseUp(mouse_button),
                    glfw::Action::Repeat => Event::Other(glfw_event),
                }
            }

            glfw::WindowEvent::Key(glfw_key, _, glfw_action, _) => {
                let key = unsafe { Key::from_glfw(glfw_key) };
                match glfw_action {
                    glfw::Action::Press => Event::KeyDown(key),
                    glfw::Action::Release => Event::KeyUp(key),
                    glfw::Action::Repeat => Event::KeyRepeat(key),
                }
            },

            _ => Event::Other(glfw_event),
        }
    }
}

/// An enum which stores a mouse button.
/// This enum is used by the `MouseDown` and `MouseUp` `Event` variants.
#[derive(Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other,
}

impl MouseButton {
    /// Converts a `glfw::MouseButton` into a `realms::MouseButton`.
    /// You don't need to call this method. It is only used by
    /// `Event::from_glfw` to convert a glfw mouse event into a realms mouse
    /// event.
    fn from_glfw(glfw_mouse_button: glfw::MouseButton) -> MouseButton {
        match glfw_mouse_button {
            glfw::MouseButton::Button1 => MouseButton::Left,
            glfw::MouseButton::Button2 => MouseButton::Right,
            glfw::MouseButton::Button3 => MouseButton::Middle,
            _ => MouseButton::Other,
        }
    }
}

/// An enum storing a list of keyboard keys and their associated glfw
/// keycodes.
/// You can match a key event using this enum.
/// It is used by the `KeyDown`, `KeyUp` and `KeyRepeat` event variants.
#[repr(usize)]
#[derive(Debug)]
pub enum Key {
    Space = 32,
    Apostrophe = 39,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Num0 = 48,
    Num1 = 49,
    Num2 = 50,
    Num3 = 51,
    Num4 = 52,
    Num5 = 53,
    Num6 = 54,
    Num7 = 55,
    Num8 = 56,
    Num9 = 57,
    Semicolon = 59,
    Equal = 61,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftBracket = 91,
    Backslash = 92,
    RightBracket = 93,
    GraveAccent = 96,
    World1 = 161,
    World2 = 162,
    Escape = 256,
    Enter = 257,
    Tab = 258,
    Backspace = 259,
    Insert = 260,
    Delete = 261,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
    PageUp = 266,
    PageDown = 267,
    Home = 268,
    End = 269,
    CapsLock = 280,
    ScrollLock = 281,
    NumLock = 282,
    PrintScreen = 283,
    Pause = 284,
    F1 = 290,
    F2 = 291,
    F3 = 292,
    F4 = 293,
    F5 = 294,
    F6 = 295,
    F7 = 296,
    F8 = 297,
    F9 = 298,
    F10 = 299,
    F11 = 300,
    F12 = 301,
    F13 = 302,
    F14 = 303,
    F15 = 304,
    F16 = 305,
    F17 = 306,
    F18 = 307,
    F19 = 308,
    F20 = 309,
    F21 = 310,
    F22 = 311,
    F23 = 312,
    F24 = 313,
    F25 = 314,
    Keypad0 = 320,
    Keypad1 = 321,
    Keypad2 = 322,
    Keypad3 = 323,
    Keypad4 = 324,
    Keypad5 = 325,
    Keypad6 = 326,
    Keypad7 = 327,
    Keypad8 = 328,
    Keypad9 = 329,
    KeypadDecimal = 330,
    KeypadDivide = 331,
    KeypadMultiply = 332,
    KeypadSubtract = 333,
    KeypadAdd = 334,
    KeypadEnter = 335,
    KeypadEqual = 336,
    LeftShift = 340,
    LeftControl = 341,
    LeftAlt = 342,
    LeftSuper = 343,
    RightShift = 344,
    RightControl = 345,
    RightAlt = 346,
    RightSuper = 347,
    Menu = 348,
}

impl Key {
    /// This function converts a glfw keycode (usize integer) into a Realms
    /// `Key` enum variant.
    /// It is used only by the `Event::from_glfw` function: **you should not
    /// call it manually**.
    ///
    /// ## KNOWN CRASHES
    /// This function is known to SEGFAULT if the key is not known by the glfw
    /// ffi bindings. If your code crashes upon pressing a specific key, this
    /// is why.
    unsafe fn from_glfw(glfw_key: glfw::Key) -> Key {
        unsafe { std::mem::transmute(glfw_key as usize) }
    }
}

