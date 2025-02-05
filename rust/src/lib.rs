use godot::prelude::*;

pub mod player;
pub mod main_scene;
pub mod npc;

pub struct LittleGame;

#[gdextension]
unsafe impl ExtensionLibrary for LittleGame {}