//! The `shape` module contains structs and functions for drawing simple 2D
//! shapes to the screen.
//! Each shape instance will create a new `VertexBuffer` which need to
//! individually be sent to the GPU. For that reason, this module should only
//! be used for prototyping and applications where performance isn't important.
