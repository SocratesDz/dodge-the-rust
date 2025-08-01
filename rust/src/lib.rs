mod hud;
mod main_scene;
mod mob;
mod player;

use godot::prelude::*;

struct DodgeTheRustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for DodgeTheRustExtension {}
