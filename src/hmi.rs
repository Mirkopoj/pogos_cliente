use crate::CANTIDAD_DE_IMAGENES;
use crate::params_struct::ParamsStruct;

use macroquad::input::KeyCode::H;
use macroquad::input::KeyCode::J;
use macroquad::input::KeyCode::K;
use macroquad::input::KeyCode::L;

use macroquad::prelude::*;

pub fn hmi(params: &mut [ParamsStruct; CANTIDAD_DE_IMAGENES]) -> char {
    let mut ret: char = 'a';
    clear_background(WHITE);

    for sprite in params{
        match sprite.gif {
            Some(_) => {
                sprite.gif.as_ref().expect("No hay gif").draw_at(sprite.x,sprite.y);
                if sprite.animada { sprite.gif.as_mut().expect("No hay gif").tick(); }
            },
            None => {
                let macroquad_params: DrawTextureParams = DrawTextureParams {
                    dest_size: None,
                    source: None,
                    rotation: sprite.rot,
                    flip_x: false,
                    flip_y: false,
                    pivot: sprite.pivot,
                };
                draw_texture_ex(sprite.foto, sprite.x, sprite.y, WHITE, macroquad_params);
            },
        }
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
