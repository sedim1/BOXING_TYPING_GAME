use crate::game::{SH, SW};
use crate::{asteroid::*, asteroid_manager};
use raylib::prelude::*;

pub struct Bullet {
    position: Vector2,
    direction: Vector2,
    radius: f32,
    pub alive: bool,
}

impl Bullet {
    pub fn new(position: Vector2, direction: Vector2) -> Self {
        Bullet {
            position,
            direction,
            radius: 5.0,
            alive: true,
        }
    }

    fn destroy_asteroid(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        asteroid_manager: &mut Vec<Asteroid>,
        asteroid: &mut Asteroid,
    ) {
        if check_collision_circles(
            self.position,
            self.radius,
            asteroid.position,
            asteroid.radius,
        ) {
            asteroid.destroy(rl, thread, asteroid_manager);
            self.alive = false;
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.alive {
            let speed: f32 = 1200.0;
            self.position += self.direction * speed * dt;

            let out_x: bool = self.position.x < -self.radius * 2.0
                || self.position.x > SW as f32 + self.radius * 2.0;
            let out_y: bool = self.position.y < -self.radius * 2.0
                || self.position.y > SH as f32 + self.radius * 2.0;

            if out_x || out_y {
                println!("bullet outside screen");
                self.alive = false;
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.position, self.radius, Color::RED);
    }
}
