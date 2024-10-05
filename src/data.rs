//! This module contains structs, datatypes and information
//! used by Realms to store information.  
//! The structs in this module do not fit into another
//! module, but are too atomic to have their own module.


use std::time::{Duration, Instant};

/// `Color` is a simple struct to store 4 `u8`s:
///   - `r` = the **red** channel/component for the color
///   - `g` = the **green** channel/component for the color
///   - `b` = the **blue** channel/component for the color
///   - `a` = the **alpha** channel/component for the color
///
/// To construct a color, use one of these functions:
///   - `Color::rgba`
///   - `Color::rgb`
///   - `Color::new`
#[derive(Clone)]
pub struct Color {

    // The `red` component
    pub r: u8,

    // The `green` component
    pub g: u8,

    // The `blue` component
    pub b: u8,

    // The `alpha` (opacity) component
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };

    /// Construct a `Color` object from the specified red,
    /// green, blue and alpha (opacity) components.  
    /// These components should be specified as a u8 value
    /// (between 0-255).
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    
    /// Construct a `Color` object from the specified red,
    /// green and blue components.  
    /// These components should be specified as a u8 value
    /// (between 0-255).
    ///
    /// Alpha (`a`) is assumed to be `255` (fully opaque).
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 255)
    }

    /// Construct a new Color object with the color value
    /// `black`.
    ///
    /// Shorthand for `Color::rgb(0, 0, 0)`.
    pub fn new() -> Color {
        Color::rgba(0, 0, 0, 255)
    }

    /// This function mutates `self` to represent the color
    /// result if the `other` color was 'added on top of'
    /// the existing `self` color.
    ///
    /// If `other` is opaque, `self` becomes `other` (in
    /// other words, the object is set to the value of
    /// `other`).
    ///
    /// If `other` is *not* opaque, `self` is set to the
    /// original value of self, but blended with `other`.
    /// The weight of each color (how much it affects the
    /// resulting color) is determined by the `alpha`
    /// channel of `other`. If `other`'s alpha channel is
    /// lower (more transparent), the result will be more
    /// similar to the original `self`. If `other`'s alpha
    /// channel is higher (more opaque), the result will be
    /// more similar to `other`.
    ///
    /// TODO: Make this documentation more understandable!
    pub fn add_layer(&mut self, other: Color) {
        if other.a == 255 {
            *self = other.clone();
        }
        let other_alpha = other.a as f64 / 255.0;
        let old_alpha = 1.0 - other_alpha;
        self.r = (self.r as f64 * old_alpha + other.r as f64 * other_alpha) as u8;
        self.g = (self.g as f64 * old_alpha + other.g as f64 * other_alpha) as u8;
        self.b = (self.b as f64 * old_alpha + other.b as f64 * other_alpha) as u8;
    }
}

/// `GameTime` is used to handle storing and updating the
/// elapsed window time.  
/// It can be used to get the application runtime, the
/// time since the last frame (delta time), the framerate
/// (FPS), etc.
pub struct GameTime {

    /// Stores the `Duration` since the last frame.
    delta: Duration,
    
    /// Stores the `Instant` the object was created.
    startup: Instant,

    /// Stores the `Instant` the frame was updated (when `new_frame` was called).
    frame_start: Instant,
}

impl GameTime {
    /// > Note: you don't need to instantiate this struct
    /// > yourself. Use the provided `window.time` instead.
    ///
    /// Construct a new `GameTime` object.
    ///   - `delta` is initialised to 0.0.
    ///   - `startup` is initialised to the current time.
    ///   - `frame_start` is initialised to the current
    ///     time.
    pub fn new() -> GameTime {
        GameTime {
            delta: Duration::new(0, 0),
            startup: Instant::now(),
            frame_start: Instant::now()
        }
    }

    /// > Note: you don't need to call this method. It is
    /// > called automatically by `window.new_frame()`.
    ///
    /// Update the GameTime object based on the current
    /// time.
    pub fn new_frame(&mut self) {
        let now = Instant::now();
        self.delta = now - self.frame_start;
        self.frame_start = now;
    }
    
    /// Get the time since the last frame (delta time) in
    /// seconds as an `f32`.
    ///
    /// This is the time between the last two calls of
    /// `self.new_frame` (a method called automatically by
    /// `Window::new_frame`).
    pub fn delta(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    /// Get the framerate (frames/second) of the window as
    /// an `f32`.  
    /// This FPS value is calculated based on the delta
    /// time, so changes each frame. 
    ///
    /// The framerate is calculated as `1.0 / delta`.
    pub fn fps(&self) -> f32 {
        1.0 / self.delta.as_secs_f32()
    }

    /// Returns the elapsed time since the `GameTime`
    /// object was created (on `Window` creation).
    /// 
    /// This function returns a `Duration`. If you require
    /// an `f32`, use the `runtimes` function.
    pub fn runtime(&self) -> Duration {
        Instant::now() - self.startup
    }

    /// Returns the elapsed time since the `GameTime`
    /// object was created (on `Window` creation).
    /// 
    /// This function returns an `f32`. If you require a
    /// `Duration`, use the `runtime` function.
    pub fn runtimes(&self) -> f32 {
        self.runtime().as_secs_f32()
    }
}

