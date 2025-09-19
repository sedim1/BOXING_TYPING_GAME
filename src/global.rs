use crate::game::*;
use raylib::prelude::*;

pub fn wrap_around(position: &mut Vector2) {
    let max: f32 = 80.0;
    //wrap ship
    if position.x > SW as f32 + max {
        position.x = -max;
    }
    if position.x < -max {
        position.x = SW as f32 + max;
    }
    if position.y > SH as f32 + max {
        position.y = -max;
    }
    if position.y < -max {
        position.y = SH as f32 + max;
    }
}
