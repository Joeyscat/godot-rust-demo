use godot::prelude::*;

mod conn;
mod player;

struct MyExtention;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtention {}
