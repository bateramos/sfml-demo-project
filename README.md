# sfml-demo-project

To run the project `cargo run`. It will open a UI with two buttons:
 - Load image with images dependency
 - Load image with egui-extras

Currently both have the same issue:
```
thread 'main' panicked at /sfml-0.24.0/src/graphics/texture.rs:328:9:
assertion failed: x + width <= my_dims.x && y + height <= my_dims.y &&
    pixels.len() == (width * height * 4) as usize
```
