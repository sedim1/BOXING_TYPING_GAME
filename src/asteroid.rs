use raylib::{ffi::rlBlendMode, prelude::*};

use crate::{
    SH,
    asteroid_manager::AsteroidManager,
    game::SW,
    global,
    ship::{self, Ship},
};

const MAX_TIME: f64 = 8.0;

pub enum AsteroidType {
    BIG,
    MEDIUM,
    SMALL,
}

pub struct Asteroid {
    pub position: Vector2,
    sprite: Texture2D,
    direction: Vector2,
    pub state: AsteroidType,
    angle: f32,
    time: f64,
    pub alive: bool,
    pub radius: f32,
}

fn out_of_screen(sprite: &Texture2D, position: &Vector2) -> bool {
    let out_x = position.x <= -sprite.width as f32 || position.x >= sprite.width as f32 + SW as f32;
    let out_y =
        position.y <= -sprite.height as f32 || position.y >= sprite.height as f32 + SH as f32;
    out_y || out_x
}

impl Asteroid {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        position: Vector2,
        direction: Vector2,
        state: AsteroidType,
    ) -> Self {
        let radius: f32 = match state {
            AsteroidType::BIG => 40.0,
            AsteroidType::MEDIUM => 25.0,
            AsteroidType::SMALL => 10.0,
        };
        let sprite: Texture2D = match state {
            AsteroidType::BIG => rl.load_texture(thread, "Assets/A_Big.png").unwrap(),
            AsteroidType::MEDIUM => rl.load_texture(thread, "Assets/A_Medium.png").unwrap(),
            AsteroidType::SMALL => rl.load_texture(thread, "Assets/A_Small.png").unwrap(),
        };
        Asteroid {
            position,
            sprite,
            direction,
            state,
            angle: 0.0,
            time: rl.get_time(),
            alive: true,
            radius,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, dt: f32) {
        if self.alive {
            let elapsed_time: f64 = rl.get_time() - self.time;
            let out: bool = out_of_screen(&self.sprite, &self.position);
            if elapsed_time >= MAX_TIME && out {
                self.alive = false;
                return;
            }
            let speed: f32 = 130.0 * dt;
            self.angle += 80.0 * dt;
            if self.angle < 0.0 {
                self.angle += 360.0;
            } else if self.angle >= 360.0 {
                self.angle -= 360.0;
            }

            self.position += self.direction * speed;
            global::wrap_around(&mut self.position);
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let width: f32 = self.sprite.width as f32;
        let height: f32 = self.sprite.height as f32;
        let origin: Vector2 = Vector2::new(width / 2.0, height / 2.0);
        let src_rect: Rectangle = Rectangle::new(0.0, 0.0, width, height);
        let dest_rect: Rectangle = Rectangle::new(self.position.x, self.position.y, width, height);
        d.draw_texture_pro(
            &self.sprite,
            src_rect,
            dest_rect,
            origin,
            self.angle,
            Color::WHITE,
        );
    }

    pub fn crashes_with_ship(&self, ship: &Ship) -> bool {
        check_collision_circles(self.position, self.radius, ship.position, ship.radius)
    }
}
