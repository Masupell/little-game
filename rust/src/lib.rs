use godot::prelude::*;

pub struct LittleGame;

#[gdextension]
unsafe impl ExtensionLibrary for LittleGame {}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Main
{
    base: Base<Node2D>
}

#[godot_api]
impl INode2D for Main
{
    fn init(base: Base<Node2D>) -> Self
    {
        Self
        {
            base
        }
    }
}