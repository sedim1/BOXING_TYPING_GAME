use crate::bullet_manager::*;
use crate::game::{SH, SH2, SW, SW2};
use crate::global;
use raylib::prelude::*;

pub enum PlayerStates {
    ALIVE,
    DEAD,
}

pub struct Ship {
    pub position: Vector2,
    pub sprite: Texture2D,
    pub angle: f32,
    pub speed: f32,
    pub radius: f32,
    pub state: PlayerStates,
    pub bullet_shooter: BulletManager,
}

impl Ship {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Ship {
            position: Vector2::new(SW2, SH2),
            sprite: rl.load_texture(thread, "Assets/Ship.png").unwrap(),
            angle: 0.0,
            speed: 0.0,
            radius: 15.0,
            state: PlayerStates::ALIVE,
            bullet_shooter: BulletManager::new(),
        }
    }

    fn rotate_player(&mut self, rl: &mut RaylibHandle, dt: f32) {
        let rot_speed: f32 = 180.0;
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.angle -= rot_speed * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.angle += rot_speed * dt;
        }
        if self.angle < 0.0 {
            self.angle += 360.0;
        } else if self.angle >= 360.0 {
            self.angle -= 360.0;
        }
    }

    fn move_player(&mut self, rl: &mut RaylibHandle, dt: f32) {
        let max_speed: f32 = 500.0;
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.speed = raylib::math::lerp(self.speed, max_speed, dt);
        } else if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.speed = raylib::math::lerp(self.speed, -max_speed, dt);
        } else {
            self.speed = raylib::math::lerp(self.speed, 0.0, dt);
        }

        let player_direction: Vector2 = Vector2::new(
            f32::cos(self.angle.to_radians()),
            f32::sin(self.angle.to_radians()),
        );

        self.position += player_direction * self.speed * dt;
        global::wrap_around(&mut self.position);
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, dt: f32) {
        self.rotate_player(rl, dt);
        self.move_player(rl, dt);
        if rl.is_key_pressed(KeyboardKey::KEY_Z) {
            self.bullet_shooter
                .ShootBullet(self.position.clone(), self.angle);
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        match self.state {
            PlayerStates::ALIVE => {
                let width: f32 = self.sprite.width as f32;
                let height: f32 = self.sprite.height as f32;
                let origin: Vector2 = Vector2::new(width / 2.0, height / 2.0);
                let src_rect: Rectangle = Rectangle::new(0.0, 0.0, width, height);
                let dest_rect: Rectangle =
                    Rectangle::new(self.position.x, self.position.y, width, height);
                d.draw_texture_pro(
                    &self.sprite,
                    src_rect,
                    dest_rect,
                    origin,
                    self.angle,
                    Color::WHITE,
                );
            }
            PlayerStates::DEAD => {}
        }
    }
}
