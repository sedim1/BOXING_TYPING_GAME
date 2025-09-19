use crate::asteroid_manager::AsteroidManager;
use crate::ship::*;
use raylib::prelude::*;

#[warn(dead_code)]
pub const SW: i32 = 800;
pub const SH: i32 = 640;
pub const SW2: f32 = SW as f32 / 2.0;
pub const SH2: f32 = SH as f32 / 2.0;

pub struct Game {
    player: Ship,
    asteroid_manager: AsteroidManager,
    rl: RaylibHandle,
    thread: RaylibThread,
}

impl Game {
    pub fn new(name: &str) -> Self {
        let (mut rl, thread) = raylib::init().title(name).size(SW, SH).vsync().build();
        let player: Ship = Ship::new(&mut rl, &thread);
        let asteroid_manager: AsteroidManager = AsteroidManager::new(&mut rl);
        Game {
            player,
            asteroid_manager,
            rl,
            thread,
        }
    }

    pub fn main_loop(&mut self) {
        while !self.rl.window_should_close() {
            let delta_time: f32 = self.rl.get_frame_time();
            self.update(delta_time);
            self.render();
        }
    }

    fn update(&mut self, dt: f32) {
        self.asteroid_manager
            .update_manager(&mut self.rl, &self.thread, dt);
        self.player.update(&mut self.rl, dt);
        self.player.bullet_shooter.update_bullets(dt);
        self.asteroid_manager
            .process_collisions_with_ship(&mut self.player);
    }

    fn render(&mut self) {
        let fps = self.rl.get_fps();
        let fps_str = fps.to_string();
        let mut d: RaylibDrawHandle = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);
        self.player.draw(&mut d);
        self.player.bullet_shooter.draw_bullets(&mut d);
        self.asteroid_manager.draw_asteroids(&mut d);
        d.draw_text(fps_str.as_str(), 0, 0, 50, Color::WHITE);
    }
}
