use crate::params_struct::ParamsStruct;
use crate::RUEDA_DESP;

use macroquad::input::KeyCode::H;
use macroquad::input::KeyCode::J;
use macroquad::input::KeyCode::K;
use macroquad::input::KeyCode::L;

use macroquad::prelude::*;

pub fn hmi(params: [ParamsStruct; 4]) -> char {
    let mut ret: char = 'a';
    clear_background(WHITE);

    let rueda = params[0];

    let macroquad_params: DrawTextureParams = DrawTextureParams {
        dest_size: None,
        source: None,
        rotation: rueda.rot,
        flip_x: false,
        flip_y: false,
        pivot: None,
    };
    draw_texture_ex(rueda.foto, rueda.x + RUEDA_DESP, rueda.y, WHITE, macroquad_params);

    for sprite in params{
        let macroquad_params: DrawTextureParams = DrawTextureParams {
            dest_size: None,
            source: None,
            rotation: sprite.rot,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
        draw_texture_ex(sprite.foto, sprite.x, sprite.y, WHITE, macroquad_params);
    }

    if let Some(key) = get_last_key_pressed() {
        if key == H {
            ret = 'h';
        }
        if key == J {
            ret = 'j';
        }
        if key == K {
            ret = 'k';
        }
        if key == L {
            ret = 'l';
        }
    }
    ret
}
