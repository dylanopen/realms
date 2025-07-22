//! The `input` module stores the `Event` enum. The events from the current
//! frame can be accessed by calling the `events()` method on a `Window`
//! instance.
//!
//! The `input` module is also used to collect input from the user through
//! the keyboard and mouse. This uses the `Key` and `MouseButton` enums.
//!

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
#[non_exhaustive]
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
    /// - $1: the `MouseButton` that was pressed. Can be `Left`, `Right` or
    ///   `Middle`. See the `MouseButton` enum for more info.
    MouseDown(MouseButton),

    /// The specified `MouseButton` was **released**
    /// This event is called when the user stops pressing down one of the mouse
    /// buttons.
    /// 
    /// ## Parameters:
    /// - $1: the `MouseButton` that was released. Can be `Left`, `Right`,
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
    #[inline]
    #[must_use]
    pub fn from_glfw(glfw_event: glfw::WindowEvent) -> Event {
        #[expect(clippy::wildcard_enum_match_arm, reason = "if more WindowEvent variants are added to glfw, we still want to ignore them (for now at least)")]
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

            glfw::WindowEvent::Focus(is_focused)
                => if is_focused { Event::Focus } else {Event::Unfocus}

            glfw::WindowEvent::CursorEnter(just_entered)
                => if just_entered { Event::CursorEnter } else { Event::CursorExit }

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
#[non_exhaustive]
pub enum MouseButton {

    /// The *left* mouse button. This may change depending on the user's
    /// system settings (some people swap round the left and right buttons).
    Left,

    /// The *middle* mouse button, usually the *scroll wheel*.
    /// On some mice, however, such as the Apple Magic Mouse, this must be
    /// triggered by gestures, such as clicking with three fingers.
    Middle,

    /// The *right* mouse button. This may change depending on the user's
    /// system settings (some people swap round the left and right buttons).
    Right,

    /// Another mouse button that is not yet supported by Realms (but may be
    /// in the future, hence the `#[non_exhaustive]` flag).
    Other,
}

impl MouseButton {
    /// Converts a `glfw::MouseButton` into a `realms::MouseButton`.
    /// You don't need to call this method. It is only used by
    /// `Event::from_glfw` to convert a glfw mouse event into a realms mouse
    /// event.
    #[inline]
    const fn from_glfw(glfw_mouse_button: glfw::MouseButton) -> MouseButton {
        #[expect(clippy::wildcard_enum_match_arm, reason = "if more variants are added, we still want to ignore them")]
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
#[non_exhaustive]
pub enum Key {

    /// The `SPACE` key. Has the keycode `32`.
    Space = 32,

    /// The `'` key. Has the keycode `39`.
    Apostrophe = 39,

    /// The `,` key. Has the keycode `44`.
    Comma = 44,

    /// The `-` key. Has the keycode `45`.
    Minus = 45,

    /// The `.` key. Has the keycode `46`.
    Period = 46,

    /// The `/` key. Has the keycode `47`.
    Slash = 47,

    /// The `0` key on the **regular keyboard* (NOT the keypad on the right of
    /// some keyboards.). Has the keycode `48`.
    Num0 = 48,

    /// The `1` key on the **regular keyboard* (NOT the keypad on the right of
    /// some keyboards.). Has the keycode `49`.
    Num1 = 49,

    /// The `2` key on the **regular keyboard* (NOT the keypad on the right of
    /// some keyboards.). Has the keycode `50`.
    Num2 = 50,

    /// The `3` key on the **regular keyboard* (NOT the keypad on the right of
    /// some keyboards.). Has the keycode `51`.
    Num3 = 51,

    /// The `4` key on the **regular keyboard* (NOT the keypad on the right of
    /// some keyboards.). Has the keycode `52`.
    Num4 = 52,

    /// The `5` key on the **regular keyboard* (NOT the keypad on the right of
    /// some keyboards.). Has the keycode `53`.
    Num5 = 53,

    /// The `6` key on the **regular keyboard* (NOT the keypad on the right of
    /// some keyboards.). Has the keycode `54`.
    Num6 = 54,

    /// The `7` key on the **regular keyboard* (NOT the keypad on the right of
    /// some keyboards.). Has the keycode `55`.
    Num7 = 55,

    /// The `8` key on the **regular keyboard* (NOT the keypad on the right of
    /// some keyboards.). Has the keycode `56`.
    Num8 = 56,

    /// The `9` key on the **regular keyboard* (NOT the keypad on the right of
    /// some keyboards.). Has the keycode `57`.
    Num9 = 57,

    /// The `;` key. Has the keycode `59`.
    Semicolon = 59,

    /// The `=` key. Has the keycode `61`.
    Equal = 61,

    /// The `A` key. Has the keycode `65`.
    A = 65,

    /// The `B` key. Has the keycode `66`.
    B = 66,

    /// The `C` key. Has the keycode `67`.
    C = 67,

    /// The `D` key. Has the keycode `68`.
    D = 68,

    /// The `E` key. Has the keycode `69`.
    E = 69,

    /// The `F` key. Has the keycode `70`.
    F = 70,

    /// The `G` key. Has the keycode `71`.
    G = 71,

    /// The `H` key. Has the keycode `72`.
    H = 72,

    /// The `I` key. Has the keycode `73`.
    I = 73,

    /// The `J` key. Has the keycode `74`.
    J = 74,

    /// The `K` key. Has the keycode `75`.
    K = 75,

    /// The `L` key. Has the keycode `76`.
    L = 76,

    /// The `M` key. Has the keycode `77`.
    M = 77,

    /// The `N` key. Has the keycode `78`.
    N = 78,

    /// The `O` key. Has the keycode `79`.
    O = 79,

    /// The `P` key. Has the keycode `80`.
    P = 80,

    /// The `Q` key. Has the keycode `81`.
    Q = 81,

    /// The `R` key. Has the keycode `82`.
    R = 82,

    /// The `S` key. Has the keycode `83`.
    S = 83,

    /// The `T` key. Has the keycode `84`.
    T = 84,

    /// The `U` key. Has the keycode `85`.
    U = 85,

    /// The `V` key. Has the keycode `86`.
    V = 86,

    /// The `W` key. Has the keycode `87`.
    W = 87,

    /// The `X` key. Has the keycode `88`.
    X = 88,

    /// The `Y` key. Has the keycode `89`.
    Y = 89,

    /// The `Z` key. Has the keycode `90`.
    Z = 90,

    /// The `[` key. Has the keycode `91`.
    LeftBracket = 91,

    /// The `\` key. Has the keycode `92`.
    Backslash = 92,

    /// The `]` key. Has the keycode `93`.
    RightBracket = 93,

    /// The \` key. Has the keycode `96`.
    GraveAccent = 96,

    /// A key on the keyboard (don't know what the world keys are, make a PR
    /// if you know!) Has the keycode `161`.
    World1 = 161,

    /// A key on the keyboard (don't know what the world keys are, make a PR
    /// if you know!) Has the keycode `162`.
    World2 = 162,

    /// The `ESC` key. Has the keycode `256`.
    Escape = 256,

    /// The `ENTER` key. Has the keycode `257`.
    Enter = 257,

    /// The `TAB` key. Has the keycode `258`.
    Tab = 258,

    /// The `BACKSPACE` key. Has the keycode `259`.
    Backspace = 259,

    /// The `INSERT` key. Has the keycode `260`.
    Insert = 260,

    /// The `DELETE` key. Has the keycode `261`.
    Delete = 261,

    /// The `RIGHT ARROW` key. Has the keycode `262`.
    Right = 262,

    /// The `LEFT ARROW` key. Has the keycode `263`.
    Left = 263,

    /// The `DOWN ARROW` key. Has the keycode `264`.
    Down = 264,

    /// The `UP ARROW` key. Has the keycode `265`.
    Up = 265,

    /// The `PAGE UP` key. Has the keycode `266`.
    PageUp = 266,

    /// The `PAGE DOWN` key. Has the keycode `267`.
    PageDown = 267,

    /// The `HOME` key. Has the keycode `268`.
    Home = 268,

    /// The `END` key. Has the keycode `269`.
    End = 269,

    /// The `CAPS LOCK` key. Has the keycode `280`.
    CapsLock = 280,

    /// The `SCROLL LOCK` key. Has the keycode `281`.
    ScrollLock = 281,

    /// The `NUM LOCK` key. Has the keycode `282`.
    NumLock = 282,

    /// The `PRINT SCREEN` key. Has the keycode `283`.
    PrintScreen = 283,

    /// The `PLAY/PAUSE` key. Has the keycode `284`.
    Pause = 284,

    /// The `F1` key. Has the keycode `290`.
    F1 = 290,

    /// The `F2` key. Has the keycode `291`.
    F2 = 291,

    /// The `F3` key. Has the keycode `292`.
    F3 = 292,

    /// The `F4` key. Has the keycode `293`.
    F4 = 293,

    /// The `F5` key. Has the keycode `294`.
    F5 = 294,

    /// The `F6` key. Has the keycode `295`.
    F6 = 295,

    /// The `F7` key. Has the keycode `296`.
    F7 = 296,

    /// The `F8` key. Has the keycode `297`.
    F8 = 297,

    /// The `F9` key. Has the keycode `298`.
    F9 = 298,

    /// The `F10` key. Has the keycode `299`.
    F10 = 299,

    /// The `F11` key. Has the keycode `300`.
    F11 = 300,

    /// The `F12` key. Has the keycode `301`.
    F12 = 301,

    /// The `F13` key. Has the keycode `302`.
    F13 = 302,

    /// The `F14` key. Has the keycode `303`.
    F14 = 303,

    /// The `F15` key. Has the keycode `304`.
    F15 = 304,

    /// The `F16` key. Has the keycode `305`.
    F16 = 305,

    /// The `F17` key. Has the keycode `306`.
    F17 = 306,

    /// The `F18` key. Has the keycode `307`.
    F18 = 307,

    /// The `F19` key. Has the keycode `308`.
    F19 = 308,

    /// The `F20` key. Has the keycode `309`.
    F20 = 309,

    /// The `F21` key. Has the keycode `310`.
    F21 = 310,

    /// The `F22` key. Has the keycode `311`.
    F22 = 311,

    /// The `F23` key. Has the keycode `312`.
    F23 = 312,

    /// The `F24` key. Has the keycode `313`.
    F24 = 313,

    /// The `F25` key. Has the keycode `314`.
    F25 = 314,

    /// The `0` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `320`.
    Keypad0 = 320,

    /// The `1` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `321`.
    Keypad1 = 321,

    /// The `2` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `322`.
    Keypad2 = 322,

    /// The `3` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `323`.
    Keypad3 = 323,

    /// The `4` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `324`.
    Keypad4 = 324,

    /// The `5` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `325`.
    Keypad5 = 325,

    /// The `6` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `326`.
    Keypad6 = 326,

    /// The `7` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `327`.
    Keypad7 = 327,

    /// The `8` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `328`.
    Keypad8 = 328,

    /// The `9` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `329`.
    Keypad9 = 329,

    /// The `.` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `330`.
    KeypadDecimal = 330,

    /// The `/` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `331`.
    KeypadDivide = 331,

    /// The `*` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `332`.
    KeypadMultiply = 332,

    /// The `-` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `333`.
    KeypadSubtract = 333,

    /// The `+` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `334`.
    KeypadAdd = 334,

    /// The `ENTER` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `335`.
    KeypadEnter = 335,

    /// The `=` key **on the keypad** (not the top row of the regular
    /// keyboard). Has the keycode `336`.
    KeypadEqual = 336,

    /// The `LEFT SHIFT` key. Has the keycode `340`.
    LeftShift = 340,

    /// The `LEFT CONTROL` key. Has the keycode `341`.
    LeftControl = 341,

    /// The `LEFT ALT` key. Has the keycode `342`.
    LeftAlt = 342,

    /// The `LEFT SUPER` (windows) key. Has the keycode `343`.
    LeftSuper = 343,

    /// The `RIGHT SHIFT` key. Has the keycode `344`.
    RightShift = 344,

    /// The `RIGHT CONTROL` key. Has the keycode `345`.
    RightControl = 345,

    /// The `RIGHT ALT` key. Has the keycode `346`.
    RightAlt = 346,

    /// The `RIGHT SUPER` (windows) key. Has the keycode `347`.
    RightSuper = 347,

    /// The `MENU` key. Has the keycode `348`.
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
        use core::mem;
        #[expect(clippy::as_conversions, reason = "can't find a way other than `as` to convert an enum variant to its respective integer")]
        unsafe { mem::transmute(glfw_key as usize) }
    }
}

