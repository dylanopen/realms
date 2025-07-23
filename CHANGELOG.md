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

- Fixed some lints in `Shader::load_str` method
- `Shader::load_str` function now takes `source` as `&str` instead of `String`
  -- BREAKING
- Deleted unused function `Color::add_layer` -- BREAKING
- Update examples to use new `Shader::load_str`

