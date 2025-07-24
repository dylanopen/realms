# Changelog for Realms

## 1.1.1 -> 1.2.1 (minor)

- Add the `NONE` constant to the `ShaderProgram` struct
  - [https://docs.rs/realms/latest/realms/shader/struct.ShaderProgram.html#associatedconstant.NONE]

## 1.2.1 -> 1.2.2 (patch)

- Add documentation to all items
- Fix many clippy lint warnings (improve backend codebase)
- No API changes

## 1.2.2 -> 1.2.3 (patch)

- Now uses all clippy lints, with some explicitly ignored 
- Fix clippy lints to improve backend codebase
- `Texture` now uses *Nearest scaling* (makes textures pixelated rather than
  blurry)
- No API changes
- Add window example (1) to README.md

## 1.2.3 -> 1.3.3 (minor)

- Add `VertexBuffer::set_layout` method
  - [https://docs.rs/realms/1.2.3/realms/vertex/struct.VertexBuffer.html#method.set_layout]
- Update examples to use `VertexBuffer::set_layout` method
- No breaking changes

## 1.3.3 -> 2.3.3 (major, breaking)

- Deleted unused function `Color::add_layer` -- BREAKING
- `Shader::load_str` function now takes `source` as `&str` instead of `String`
  -- BREAKING

Non-breaking changes:

- Fixed some lints in `Shader::load_str` method
- Update examples to use new `Shader::load_str`

## 2.3.3 -> 2.3.4 (patch)

- Updated the `image` dependency from `0.19.0` to `0.25.6`
- Removed some `image` features, reducing dependencies from `58` to `37`,
  leading to faster compilation times
- No API changes

## 2.3.4 -> 3.3.4 (major, breaking)

- Changed method signature of `Window::new_frame(&mut self, &ShaderProgram)` to
  `Window::new_frame(&mut self)` (no longer takes a reference to the
  `ShaderProgram`) -- BREAKING
- Changed method signature of `VertexBuffer::draw(&self)` to
  `VertexBuffer::draw(&self, &ShaderProgram)` -- BREAKING

Non-breaking changes:

- Fixed broken code example in documentation for `Shader` and `ShaderProgram`
  structs (no longer convert to owned strings using `.to_string()`)
- Fixed missing 'a' component when listed in `VertexBuffer::set_layout`
  documentation
- Update examples to use the new way of binding shaders

## 3.3.4 -> 3.4.4 (minor)

- Add `shape` module
- Add builtin shader `shape2d.vert.glsl` (vertex)
- Add builtin shader `shape2d.frag.glsl` (fragment)
- Add `shape2d_shader` function for loading builtin shape2d shaders
- Add `TriangleShape` struct for drawing 3 points more easily

