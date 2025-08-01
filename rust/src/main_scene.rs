use godot::{
    classes::{Marker2D, PathFollow2D, Timer},
    global::{randf, randf_range},
    prelude::*,
};

use crate::{mob::Mob, player::Player};

#[derive(GodotClass)]
#[class(base = Node, init)]
struct Main {
    #[export]
    mob_scene: Option<Gd<PackedScene>>,

    #[init(node = "Player")]
    player: OnReady<Gd<Player>>,

    #[init(node = "MobTimer")]
    mob_timer: OnReady<Gd<Timer>>,

    #[init(node = "ScoreTimer")]
    score_timer: OnReady<Gd<Timer>>,

    #[init(node = "StartTimer")]
    start_timer: OnReady<Gd<Timer>>,

    #[init(node = "StartPosition")]
    start_position: OnReady<Gd<Marker2D>>,

    #[init(node = "MobPath/MobSpawnLocation")]
    mob_spawn_location: OnReady<Gd<PathFollow2D>>,

    score: u32,

    base: Base<Node>,
}

#[godot_api]
impl Main {
    #[func]
    fn game_over(&mut self) {
        self.mob_timer.stop();
        self.score_timer.stop();
    }

    #[func]
    fn new_game(&mut self) {
        self.player
            .bind_mut()
            .start(self.start_position.get_position());

        self.start_timer.start();
    }

    #[func]
    fn on_mob_timer_timeout(&mut self) {
        let scene = self.mob_scene.clone().unwrap();
        let mut mob = scene.instantiate_as::<Mob>();

        self.mob_spawn_location.set_progress_ratio(randf() as f32);

        let mut direction = self.mob_spawn_location.get_rotation() + std::f32::consts::PI / 2.0;

        mob.set_position(self.mob_spawn_location.get_position());

        direction += randf_range(-std::f64::consts::PI / 4.0, std::f64::consts::PI / 4.0) as f32;
        mob.set_rotation(direction);

        let velocity = Vector2::new(randf_range(150.0, 250.0) as f32, 0.0);
        mob.set_linear_velocity(velocity.rotated(direction));

        self.base_mut().add_child(&mob);
    }

    #[func]
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;
    }

    #[func]
    fn on_start_timer_timeout(&mut self) {
        self.mob_timer.start();
        self.score_timer.start();
    }
}

#[godot_api]
impl INode for Main {
    fn ready(&mut self) {
        self.player
            .signals()
            .hit()
            .connect_other(self, Self::game_over);

        self.mob_timer
            .signals()
            .timeout()
            .connect_other(self, Self::on_mob_timer_timeout);

        self.start_timer
            .signals()
            .timeout()
            .connect_other(self, Self::on_start_timer_timeout);

        self.score_timer
            .signals()
            .timeout()
            .connect_other(self, Self::on_score_timer_timeout);
    }
}
