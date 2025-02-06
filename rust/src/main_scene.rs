use godot::{classes::{CharacterBody2D, ColorRect, InputEvent, Viewport}, prelude::*};

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

#[godot_api]
impl Main
{
    #[func]
    fn mouse_over_npc(&mut self, _viewport: Gd<Viewport>, event: Gd<InputEvent>, _shape_idx: i64)
    {
        if event.is_action_pressed("mouse_right")
        {
            let npc = self.base().get_node_as::<CharacterBody2D>("NPC");
            let mut text_box = npc.get_node_as::<ColorRect>("ColorRect");
            if text_box.is_visible()
            {
                text_box.hide();
            }
            else 
            {
                text_box.show();   
            }
        }
    }
}