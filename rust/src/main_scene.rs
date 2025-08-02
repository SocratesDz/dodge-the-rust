use godot::classes::{AudioStreamPlayer, Marker2D, PathFollow2D, Timer};
use godot::global::{randf, randf_range};
use godot::prelude::*;

use crate::{hud::HUD, mob::Mob, player::Player};

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

    #[init(node = "HUD")]
    hud: OnReady<Gd<HUD>>,

    #[init(node = "Music")]
    music: OnReady<Gd<AudioStreamPlayer>>,

    #[init(node = "DeathSound")]
    death_sound: OnReady<Gd<AudioStreamPlayer>>,

    score: u32,

    base: Base<Node>,
}

#[godot_api]
impl Main {
    #[func]
    fn game_over(&mut self) {
        self.mob_timer.stop();
        self.score_timer.stop();
        self.hud.bind_mut().show_game_over();
        self.music.stop();
        self.death_sound.play();
    }

    #[func]
    fn new_game(&mut self) {
        self.score = 0;

        self.player
            .bind_mut()
            .start(self.start_position.get_position());

        self.start_timer.start();

        let mut hud = self.hud.bind_mut();
        hud.update_score(self.score);
        hud.show_message("Get Ready!");
        hud.base_mut()
            .get_tree()
            .unwrap()
            .call_group("mobs", "queue_free", &[]);

        self.music.play();
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
        self.hud.bind_mut().update_score(self.score);
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
        let main = self.to_gd();

        self.player
            .signals()
            .hit()
            .connect_other(&main, Self::game_over);

        self.mob_timer
            .signals()
            .timeout()
            .connect_other(&main, Self::on_mob_timer_timeout);

        self.start_timer
            .signals()
            .timeout()
            .connect_other(&main, Self::on_start_timer_timeout);

        self.score_timer
            .signals()
            .timeout()
            .connect_other(&main, Self::on_score_timer_timeout);

        self.hud
            .signals()
            .start_game()
            .connect_other(&main, Self::new_game);
    }
}
