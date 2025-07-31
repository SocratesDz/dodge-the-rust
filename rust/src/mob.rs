use godot::classes::{AnimatedSprite2D, IRigidBody2D, RigidBody2D, VisibleOnScreenNotifier2D};
use godot::global::randi;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RigidBody2D, init)]
struct Mob {
    base: Base<RigidBody2D>,
}

#[godot_api]
impl Mob {
    #[func]
    fn on_visible_on_screen_notifier_2d_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IRigidBody2D for Mob {
    fn ready(&mut self) {
        let mut animated_sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        let mob_types = animated_sprite
            .get_sprite_frames()
            .unwrap()
            .get_animation_names();
        let random_index = randi() as usize % mob_types.len();
        animated_sprite
            .play_ex()
            .name(mob_types[random_index].arg())
            .done();

        let visibility_notifier = self
            .base_mut()
            .get_node_as::<VisibleOnScreenNotifier2D>("VisibleOnScreenNotifier2D");
        visibility_notifier
            .signals()
            .screen_exited()
            .connect_other(self, Self::on_visible_on_screen_notifier_2d_screen_exited);
    }
}
