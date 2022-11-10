use macroquad::prelude::*;
use quad_gif;

use std::io::{Read, Write, ErrorKind};
use std::net::TcpStream;
use std::{thread, time};

mod params_struct;
use crate::params_struct::ParamsStruct;

mod hmi;
use crate::hmi::hmi;

extern crate modulos_comunes;
use modulos_comunes::{DataStruct, TcpMessage, EMPTYTCPMESSAGE, from_bytes};

mod procesamiento;
use crate::procesamiento::proc;

const CANTIDAD_DE_IMAGENES: usize = 8; 

const CINTA_X: f32 = 10.0; 
const CINTA_Y: f32 = 900.0; 
const CINTA_DEST_X: f32 = 1053.0; 
const CINTA_DEST_Y: f32 = 117.0; 
const CINTA_DESP: f32 = 1070.0; 

const CAMA_X: f32 = 800.0; 
const CAMA_Y: f32 = 1000.0; 
const CAMA_DEST_X: f32 = 200.0; 
const CAMA_DEST_Y: f32 = 48.0; 
const CAMA_DESP: f32 = 100.0; 

const LED_X: f32 = 1000.0; 
const LED_Y: f32 = 1100.0; 
const LED_DEST_X: f32 = 48.0; 
const LED_DEST_Y: f32 = 48.0; 

const SELECTOR_X: f32 = 2150.0; 
const SELECTOR_Y: f32 = 1000.0; 
const SELECTOR_DEST_X: f32 = 200.0; 
const SELECTOR_DEST_Y: f32 = 200.0; 

const TACHO_X: f32 = 2150.0; 
const TACHO_Y: f32 = 1200.0; 
const TACHO_DEST_X: f32 = 200.0; 
const TACHO_DEST_Y: f32 = 200.0; 

const TICK_X: f32 = 2150.0; 
const TICK_Y: f32 = 700.0; 
const TICK_DEST_X: f32 = 200.0; 
const TICK_DEST_Y: f32 = 196.0; 

#[macroquad::main("Pogos")]
async fn main() {
    match TcpStream::connect("192.168.1.250:3333") {
        Ok(mut stream) => {
            stream.set_nonblocking(true).expect("set_nonblocking failed");
            println!("Successfully connected to server in port 3333");

            let (cinta0,cinta1) = (
                quad_gif::GifAnimation::load("./images/TransportadoraChica.gif".to_string()).await,
                quad_gif::GifAnimation::load("./images/TransportadoraChica.gif".to_string()).await
            );
            /*let (cinta0,cinta1) = match screen_width() as u32 {
                800 => {
                    (
                        quad_gif::GifAnimation::load("./images/TransportadoraChica.gif".to_string()).await,
                        quad_gif::GifAnimation::load("./images/TransportadoraChica.gif".to_string()).await
                    )
                },
                _ => {(
                        quad_gif::GifAnimation::load("../images/Transportadora.gif".to_string()).await,
                        quad_gif::GifAnimation::load("../images/Transportadora.gif".to_string()).await
                    )
                },
            };*/

            let cama = Texture2D::from_file_with_format(
                include_bytes!("../images/Cama.png"),
                Some(ImageFormat::Png),
            );
            let led_apagado = Texture2D::from_file_with_format(
                include_bytes!("../images/LedApagado.png"),
                Some(ImageFormat::Png),
            );
            let led_encendido = Texture2D::from_file_with_format(
                include_bytes!("../images/LedEncendido.png"),
                Some(ImageFormat::Png),
            );
            let selector = Texture2D::from_file_with_format(
                include_bytes!("../images/Selector.png"),
                Some(ImageFormat::Png),
            );
            let tacho = Texture2D::from_file_with_format(
                include_bytes!("../images/Tacho.png"),
                Some(ImageFormat::Png),
            );
            let tick = Texture2D::from_file_with_format(
                include_bytes!("../images/Tick.png"),
                Some(ImageFormat::Png),
            );
            let mut params: [ParamsStruct; CANTIDAD_DE_IMAGENES] = [
                ParamsStruct {
                    foto: led_encendido,
                    gif: Some(cinta0),
                    rot: 0.0,
                    animada: false,
                    x: CINTA_X,
                    y: CINTA_Y,
                    pivot: None,
                    x_dest: CINTA_DEST_X,
                    y_dest: CINTA_DEST_Y,
                },
                ParamsStruct {
                    foto: led_encendido,
                    gif: Some(cinta1),
                    rot: 0.0,
                    animada: false,
                    x: CINTA_X + CINTA_DESP,
                    y: CINTA_Y,
                    pivot: None,
                    x_dest: CINTA_DEST_X,
                    y_dest: CINTA_DEST_Y,
                },
                ParamsStruct {
                    foto: cama,
                    gif: None,
                    rot: 0.0,
                    animada: false,
                    x: CAMA_X,
                    y: CAMA_Y,
                    pivot: None,
                    x_dest: CAMA_DEST_X,
                    y_dest: CAMA_DEST_Y,
                },
                ParamsStruct {
                    foto: led_apagado,
                    gif: None,
                    rot: 0.0,
                    animada: false,
                    x: LED_X,
                    y: LED_Y,
                    pivot: None,
                    x_dest: LED_DEST_X,
                    y_dest: LED_DEST_Y,
                },
                ParamsStruct {
                    foto: led_apagado,
                    gif: None,
                    rot: 0.0,
                    animada: false,
                    x: LED_X + CINTA_DESP,
                    y: LED_Y,
                    pivot: None,
                    x_dest: LED_DEST_X,
                    y_dest: LED_DEST_Y,
                },
                ParamsStruct {
                    foto: selector,
                    gif: None,
                    rot: 0.0,
                    animada: false,
                    x: SELECTOR_X,
                    y: SELECTOR_Y,
                    pivot: Some(Vec2::new(2300.0,1050.0)),
                    x_dest: SELECTOR_DEST_X,
                    y_dest: SELECTOR_DEST_Y,
                },
                ParamsStruct {
                    foto: tacho,
                    gif: None,
                    rot: 0.0,
                    animada: false,
                    x: TACHO_X,
                    y: TACHO_Y,
                    pivot: None,
                    x_dest: TACHO_DEST_X,
                    y_dest: TACHO_DEST_Y,
                },
                ParamsStruct {
                    foto: tick,
                    gif: None,
                    rot: 0.0,
                    animada: false,
                    x: TICK_X,
                    y: TICK_Y,
                    pivot: None,
                    x_dest: TICK_DEST_X,
                    y_dest: TICK_DEST_Y,
                },
            ];

            let mut sensores = DataStruct {
                cinta1: false,
                cinta2: false,
                pogos: false,
                selector: false,
                sensor1: false,
                sensor2: false,
            };
            loop {
                let f_time = get_frame_time();
                proc(sensores, &mut params, led_apagado, led_encendido);
                let msg = match hmi(&mut params) {
                    'h' => {
                        b"h"
                    }
                    'j' => {
                        b"j"
                    }
                    'k' => {
                        b"k"
                    }
                    'l' => {
                        b"l"
                    }
                    _ => {
                        b"a"
                    }
                };
                if msg != b"a" {
                    stream.write(msg).expect("Stream write failed");
                }
                let mut data: TcpMessage = EMPTYTCPMESSAGE;
                match stream.read(&mut data) {
                    Ok(_) => {
                        sensores = from_bytes(&data);
                    }
                    Err(e) => {
                        if e.kind() != ErrorKind::WouldBlock {
                            println!("Failed to receive data: {}", e);
                        }
                    }
                }
                let rest = (1./60.)-f_time;
                thread::sleep(time::Duration::from_secs_f32(
                        if rest > 0.0 {
                            rest
                        } else {
                            0.0
                        }
                        ));
                next_frame().await;
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
