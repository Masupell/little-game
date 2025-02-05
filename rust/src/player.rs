use godot::{classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D}, prelude::*};

// const SPEED: f32 = 700.0; // # Base horizontal movement speed

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player
{
    base: Base<CharacterBody2D>,
    #[export]
    speed: f32
}

#[godot_api]
impl ICharacterBody2D for Player
{
    fn init(base: Base<CharacterBody2D>) -> Self
    {
        Self
        {
            base,
            speed: 20.0
        }
    }

    fn ready(&mut self)
    {
        godot_print!("Player Created");
        let sprite = self.base().get_node_as::<AnimatedSprite2D>("PlayerSprite");

        let sprite_frames = sprite.get_sprite_frames().unwrap();
        let name = sprite.get_animation();
        let frame = sprite.get_frame();
        let texture = sprite_frames.get_frame_texture(&name, frame).unwrap();

        let size = texture.get_size();
        let scale = sprite.get_scale();
        let mut material = sprite.get_material().unwrap();
        material.set("shader_parameter/size", &Variant::from(size*scale));

        self.base_mut().set_process_priority(1);
        self.base_mut().set_process(true);
    }

    fn physics_process(&mut self, dt: f64)
    {
        let input = Input::singleton();
        let speed = self.speed;

        let direction = input.get_vector("left", "right", "up", "down");

        self.base_mut().set_velocity(direction * speed * 1000.0 * dt as f32);

        self.base_mut().move_and_slide();
        

        let mut sprite = self.base().get_node_as::<AnimatedSprite2D>("PlayerSprite");
        // let mut material = sprite.get_material().unwrap();
        // let mut anim = self.base().get_node_as::<AnimationPlayer>("AnimationPlayer");

        // let frame_time = anim.get_current_animation_position();
        // let frame = ((frame_time * 10.0) as i32) as f32;
        // material.set("shader_parameter/frame", &Variant::from(frame));

        // if input.is_action_just_pressed("mouse_left")
        // {
        //     anim.pause();
        // }
        // if input.is_action_just_pressed("mouse_right")
        // {
        //     anim.play();
        // }

        if direction.x != 0.0 || direction.y != 0.0
        {
            sprite.play();
        }
        else
        {
            sprite.pause();
        }

        if direction.x < 0.0
        {
            sprite.set_flip_h(true);
        }
        else if direction.x > 0.0
        {
            sprite.set_flip_h(false);
        }

        // let anim = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        // let anim_name = anim.get_animation();
        // let sprite_frames = anim.get_sprite_frames();
        // let frame = anim.get_frame();
        // let mut material = anim.get_material().unwrap();
        // godot_print!("Frame: {}", frame);
        // material.set("shader_parameter/frame", &Variant::from(frame));
    }

    fn process(&mut self, _: f64) 
    {
        let anim = self.base().get_node_as::<AnimatedSprite2D>("PlayerSprite");
        let frame = anim.get_frame();
        let mut material = anim.get_material().unwrap();
        material.set("shader_parameter/frame", &Variant::from(frame));
    }
}

// fn set_shader_param(mut material: Gd<ShaderMaterial>, param_name: &str, value: Variant) 
// {
//     material.set(format!("shader_parameter/{}", param_name), &value);
// }