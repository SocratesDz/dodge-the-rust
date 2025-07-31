use godot::classes::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, Input};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Player {
    #[export]
    speed: f32,
    screen_size: Vector2,
    base: Base<Area2D>,
}

#[godot_api]
impl Player {
    #[signal]
    fn hit();

    #[func]
    fn on_player_body_entered(&mut self, _body: Gd<Node2D>) {
        self.base_mut().hide();
        self.signals().hit().emit();
        self.base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D")
            .set_deferred("disabled", &true.to_variant());
    }

    #[func]
    fn start(&mut self, position: Vector2) {
        self.base_mut().set_position(position);
        self.base_mut().show();
        let mut collision_shape = self
            .base_mut()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");
        collision_shape.set_disabled(false);
    }
}

#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            speed: 400.0,
            screen_size: Vector2 { x: 0.0, y: 0.0 },
            base,
        }
    }

    fn ready(&mut self) {
        self.screen_size = self.base().get_viewport_rect().size;
        self.signals()
            .body_entered()
            .connect_self(Self::on_player_body_entered);
        self.base_mut().hide();
    }

    fn process(&mut self, delta: f64) {
        let mut animated_sprite = self
            .base_mut()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        let mut velocity = Vector2::ZERO;
        let input = Input::singleton();

        let input_velocity = input.get_vector("move_left", "move_right", "move_up", "move_down");
        if input_velocity.length() > 0.0 {
            velocity = input_velocity.normalized() * self.speed * delta as f32;
            animated_sprite.play();
        } else {
            animated_sprite.stop();
        }
        let old_position = self.base_mut().get_global_position() + velocity;
        let new_position = old_position.clamp(Vector2::ZERO, self.screen_size);
        self.base_mut().set_global_position(new_position);

        // Animation
        if velocity.x != 0.0 {
            animated_sprite.set_animation("walk");
            animated_sprite.set_flip_v(false);
            animated_sprite.set_flip_h(velocity.x < 0.0);
        } else if velocity.y != 0.0 {
            animated_sprite.set_animation("up");
            animated_sprite.set_flip_v(velocity.y > 0.0);
        }
    }
}
