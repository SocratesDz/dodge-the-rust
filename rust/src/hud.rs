use godot::classes::{Button, CanvasLayer, ICanvasLayer, Label, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CanvasLayer, init)]
pub struct HUD {
    #[init(node = "Message")]
    message: OnReady<Gd<Label>>,

    #[init(node = "MessageTimer")]
    message_timer: OnReady<Gd<Timer>>,

    #[init(node = "StartButton")]
    start_button: OnReady<Gd<Button>>,

    #[init(node = "ScoreLabel")]
    score_label: OnReady<Gd<Label>>,
    base: Base<CanvasLayer>,
}

#[godot_api]
impl HUD {
    #[signal]
    pub fn start_game();

    pub fn show_message(&mut self, text: &str) {
        self.message.set_text(text);
        self.message.show();
        self.message_timer.start();
    }

    pub fn show_game_over(&mut self) {
        self.show_message("Game Over");
        // Ugly
        let mut timer = self.base().get_tree().unwrap().create_timer(2.0).unwrap();
        timer.connect("timeout", &self.base().callable("show_message_text"));
    }

    #[func]
    fn show_message_text(&mut self) {
        self.message.set_text("Dodge the Creeps!");
        self.message.show();

        let mut timer = self.base().get_tree().unwrap().create_timer(1.0).unwrap();
        timer.connect("timeout", &self.base().callable("show_start_button"));
    }

    #[func]
    fn show_start_button(&mut self) {
        self.start_button.show();
    }

    #[func]
    pub fn update_score(&mut self, score: u32) {
        self.score_label.set_text(&score.to_string());
    }

    fn on_start_button_pressed(&mut self) {
        self.start_button.hide();
        self.signals().start_game().emit();
    }

    fn on_message_timer_timeout(&mut self) {
        self.message.hide();
    }
}

#[godot_api]
impl ICanvasLayer for HUD {
    fn ready(&mut self) {
        self.start_button
            .signals()
            .pressed()
            .connect_other(self, Self::on_start_button_pressed);
        self.message_timer
            .signals()
            .timeout()
            .connect_other(self, Self::on_message_timer_timeout);
    }
}
