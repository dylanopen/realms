# Changelog for Realms

## 1.1.1 -> 1.2.1 (minor)

- Add the `NONE` constant to the `ShaderProgram` struct
  - [https://docs.rs/realms/latest/realms/shader/struct.ShaderProgram.html#associatedconstant.NONE]

# 1.2.1 -> 1.2.2 (patch)

- Add documentation to all items
- Fix many clippy lint warnings (improve backend codebase)
- No API changes

# 1.2.2 -> 1.2.3 (patch)

- Now uses all clippy lints, with some explicitly ignored 
- Fix clippy lints to improve backend codebase
- `Texture` now uses *Nearest scaling* (makes textures pixelated rather than
  blurry)
- No API changes
- Add window example (1) to README.md

