use crate::bullet::*;
use crate::ship::*;
use raylib::prelude::*;
use std::vec;

pub struct BulletManager {
    bullets: Vec<Bullet>,
}

impl BulletManager {
    pub fn new() -> Self {
        BulletManager { bullets: vec![] }
    }

    pub fn ShootBullet(&mut self, position: Vector2, angle: f32) {
        let position: Vector2 = position;
        let direction: Vector2 =
            Vector2::new(f32::cos(angle.to_radians()), f32::sin(angle.to_radians()));
        println!("Shoot!");
        self.bullets.push(Bullet::new(position, direction));
    }

    pub fn erase_inactive_bullets(&mut self) {
        self.bullets.retain(|bullet| bullet.alive);
    }

    pub fn update_bullets(&mut self, dt: f32) {
        for bullet in self.bullets.iter_mut() {
            bullet.update(dt);
        }
    }

    pub fn draw_bullets(&self, d: &mut RaylibDrawHandle) {
        for bullet in self.bullets.iter() {
            bullet.draw(d);
        }
    }
}
