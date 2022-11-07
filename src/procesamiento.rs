use crate::modulos_comunes::DataStruct;
use crate::params_struct::ParamsStruct;
use crate::{CAMA_Y,CAMA_DESP, CANTIDAD_DE_IMAGENES};

extern crate load_file;

use macroquad::prelude::*;

pub fn proc(sensores: DataStruct, param_ret: &mut [ParamsStruct; CANTIDAD_DE_IMAGENES], led_apagado: Texture2D, led_encendido: Texture2D) {

    param_ret[0].animada = if sensores.cinta { true } else { false };

    param_ret[1].animada = if sensores.cinta { true } else { false };

    param_ret[2].y = CAMA_Y + if sensores.pogos { CAMA_DESP } else { 0.0 };

    param_ret[3].foto = if sensores.sensor {
        led_encendido
    } else {
        led_apagado
    };

    param_ret[4].foto = if sensores.sensor {
        led_encendido
    } else {
        led_apagado
    };

    param_ret[5].rot = if sensores.selector {1.5707963268} else {0.0};
}
