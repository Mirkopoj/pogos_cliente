use crate::CANTIDAD_DE_IMAGENES;
use crate::params_struct::ParamsStruct;

use macroquad::input::KeyCode::H;
use macroquad::input::KeyCode::J;
use macroquad::input::KeyCode::K;
use macroquad::input::KeyCode::L;

use macroquad::prelude::*;

pub fn hmi(params: &mut [ParamsStruct; CANTIDAD_DE_IMAGENES]) -> char {
    let escalado_x: f32 = screen_width()/2560.0;
    let escalado_y: f32 = screen_height()/1440.0;
    
    let mut ret: char = 'a';
    clear_background(WHITE);

    for sprite in params{
        match sprite.gif {
            Some(_) => {
                sprite.gif.as_ref().expect("No hay gif").draw_at(sprite.x*escalado_x,sprite.y*escalado_y);
                if sprite.animada { sprite.gif.as_mut().expect("No hay gif").tick(); }
            },
            None => {
                let macroquad_params: DrawTextureParams = DrawTextureParams {
                    dest_size: Some(Vec2::new(sprite.x_dest*escalado_x,sprite.y_dest*escalado_y)),
                    source: None,
                    rotation: sprite.rot,
                    flip_x: false,
                    flip_y: false,
                    pivot: match sprite.pivot {
                        Some(pivote) => {
                            let ret = Vec2::new(
                                pivote.x * escalado_x,
                                pivote.y * escalado_y
                            );
                            Some(ret)
                        },
                        None => { None },
                    },
                };
                draw_texture_ex(sprite.foto, sprite.x*escalado_x, sprite.y*escalado_y, WHITE, macroquad_params);
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
