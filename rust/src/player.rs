use godot::{classes::{CharacterBody2D, ICharacterBody2D, Sprite2D}, global::move_toward, prelude::*};

const SPEED: f32 = 350.0; // # Base horizontal movement speed
const ACCELERATION: f64 = 1200.0; // # Base acceleration

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player
{
    base: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for Player
{
    fn init(base: Base<CharacterBody2D>) -> Self
    {
        Self
        {
            base
        }
    }

    fn ready(&mut self)
    {
        godot_print!("Player Created");
    }

    fn physics_process(&mut self, delta: f64)
    {
        let input = Input::singleton();

        let direction = input.get_vector("left", "right", "up", "down");

        self.base_mut().set_velocity(direction * SPEED);

        self.base_mut().move_and_slide();
    }
}