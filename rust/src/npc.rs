use godot::{classes::{CharacterBody2D, ICharacterBody2D}, prelude::*};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct NPC
{
    base: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for NPC
{
    fn init(base: Base<CharacterBody2D>) -> Self
    {
        Self
        {
            base
        }
    }
}

#[godot_api]
impl NPC
{
    // #[func]
    // fn clicked()
    // {
    //     godot_print!("NPC Clicked");
    // }
}