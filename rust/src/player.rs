use godot::{classes::{CharacterBody2D, ICharacterBody2D, ShaderMaterial, Sprite2D}, global::move_toward, prelude::*};

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
        let sprite = self.base().get_node_as::<Sprite2D>("Sprite2D");
        let size = sprite.get_texture().unwrap().get_size();
        let scale = sprite.get_scale();
        let mut material = sprite.get_material().unwrap();
        material.set("shader_parameter/test", &Variant::from(0.0));
        material.set("shader_parameter/size", &Variant::from(size*scale));
        godot_print!("Size: {:?}", size);
    }

    fn physics_process(&mut self, delta: f64)
    {
        let input = Input::singleton();

        let direction = input.get_vector("left", "right", "up", "down");

        self.base_mut().set_velocity(direction * SPEED);

        self.base_mut().move_and_slide();
        

        let sprite = self.base().get_node_as::<Sprite2D>("Sprite2D");
        let mouse_pos = sprite.get_local_mouse_position();
        if input.is_action_just_pressed("mouse_left")
        {
            godot_print!("Mouse Pos: {:?}", mouse_pos);
        }
        // godot_print!("Mouse Pos: {:?}", mouse_pos);
        // let mut material = sprite.get_material().unwrap();
        // material.set("shader_parameter/test", &Variant::from(1.0));
    }
}

// fn set_shader_param(mut material: Gd<ShaderMaterial>, param_name: &str, value: Variant) 
// {
//     material.set(format!("shader_parameter/{}", param_name), &value);
// }