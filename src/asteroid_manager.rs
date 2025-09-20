use crate::asteroid::{Asteroid, AsteroidType};
use crate::bullet;
use crate::bullet_manager::{self, BulletManager};
use crate::game::{SH, SW};
use crate::ship::*;
use rand::*;
use raylib::prelude::*;
use std::vec;

pub struct AsteroidManager {
    active_asteroids: Vec<Asteroid>,
    spawn_rate: f64,
    last_spawn: f64,
}

impl AsteroidManager {
    pub fn new(rl: &mut RaylibHandle) -> Self {
        let active_asteroids: Vec<Asteroid> = vec![];
        let last_spawn: f64 = rl.get_time();
        let spawn_rate: f64 = 0.7;
        AsteroidManager {
            active_asteroids,
            spawn_rate,
            last_spawn,
        }
    }

    fn spawn_asteroids(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let max: f32 = 70.0;
        let mut rng = rng();
        let spawn_position: i8 = rng.random_range(1..=4);
        let mut position: Vector2 = Vector2::zero();
        let mut direction: Vector2 = Vector2::zero();
        let angle: f32 = 0.0;
        let mut angle_dir: f32 = 0.0;
        //Define the position where it will spawn
        match spawn_position {
            //Top
            1 => {
                position.x = rng.random_range(-max..SW as f32 + max);
                position.y = -max;
                angle_dir = random_range(180.0..360.0);
            }
            //bottom
            2 => {
                position.x = rng.random_range(-max..SW as f32 + max);
                position.y = SH as f32 + max;
                angle_dir = random_range(0.0..180.0);
            }
            //Left
            3 => {
                position.y = rng.random_range(-max..SH as f32 + max);
                position.x = -max;
                angle_dir = random_range(-90.0..90.0);
            }
            //Right
            4 => {
                position.y = rng.random_range(-max..SH as f32 + max);
                position.x = SW as f32 + max;
                angle_dir = random_range(180.0..270.0);
            }
            i8::MIN..=0_i8 | 5_i8..=i8::MAX => todo!(),
        }
        direction.x = f32::cos(angle_dir.to_radians());
        direction.y = f32::sin(angle_dir.to_radians());
        //Define asteroid type
        let type_option: i8 = rng.random_range(1..=3);
        let state: AsteroidType = match type_option {
            1 => AsteroidType::BIG,
            2 => AsteroidType::MEDIUM,
            3 => AsteroidType::SMALL,
            i8::MIN..=0_i8 | 4_i8..=i8::MAX => todo!(),
        };
        self.active_asteroids
            .push(Asteroid::new(rl, thread, position, direction, state));
    }

    fn erase_inactive_asteroids(&mut self) {
        self.active_asteroids.retain(|asteroid| asteroid.alive);
    }

    fn update_asteroids(&mut self, rl: &mut RaylibHandle, dt: f32) {
        for asteroid in self.active_asteroids.iter_mut() {
            asteroid.update(rl, dt);
        }
    }

    pub fn draw_asteroids(&self, d: &mut RaylibDrawHandle) {
        for asteroid in self.active_asteroids.iter() {
            asteroid.draw(d);
        }
    }

    pub fn process_collisions_with_bullets(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        bullet_manager: &mut BulletManager,
    ) {
        for i in 0..self.active_asteroids.len() {
            for j in 0..bullet_manager.bullets.len() {
                if check_collision_circles(
                    self.active_asteroids[i].position,
                    self.active_asteroids[i].radius,
                    bullet_manager.bullets[j].position,
                    bullet_manager.bullets[j].radius,
                ) {
                    //Destroy current asteroid and bullet and add 3 more if possible
                    bullet_manager.bullets[j].alive = false;
                    self.active_asteroids[i].alive = false;
                    let mut angle: f32 = 45.0;
                    for x in 0..4 {
                        let position: Vector2 = self.active_asteroids[i].position.clone();
                        let direction: Vector2 = Vector2::new(
                            f32::cos(angle.to_radians()),
                            f32::sin(angle.to_radians()),
                        );
                        let state: AsteroidType = match self.active_asteroids[i].state {
                            AsteroidType::BIG => AsteroidType::MEDIUM,
                            AsteroidType::MEDIUM => AsteroidType::SMALL,
                            AsteroidType::SMALL => return,
                        };
                        self.active_asteroids
                            .push(Asteroid::new(rl, thread, position, direction, state));
                        angle += 90.0;
                    }
                }
            }
        }
    }

    pub fn process_collisions_with_ship(&mut self, ship: &mut Ship) {
        match ship.state {
            PlayerStates::ALIVE => {
                for asteroid in self.active_asteroids.iter_mut() {
                    if asteroid.crashes_with_ship(ship) {
                        asteroid.alive = false;
                        ship.state = PlayerStates::DEAD;
                        return;
                    }
                }
            }
            PlayerStates::DEAD => {}
        }
    }

    pub fn update_manager(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, dt: f32) {
        let current_time = rl.get_time();
        let elapsed_time = current_time - self.last_spawn;
        self.erase_inactive_asteroids();
        if elapsed_time >= self.spawn_rate {
            self.last_spawn = current_time;
            self.spawn_asteroids(rl, thread);
        }
        self.update_asteroids(rl, dt);
    }
}
