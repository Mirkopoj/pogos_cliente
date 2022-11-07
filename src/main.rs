use macroquad::prelude::*;
use quad_gif;

use std::io::{Read, Write, ErrorKind};
use std::net::TcpStream;
use std::str::from_utf8;
use std::{thread, time};

mod params_struct;
use crate::params_struct::ParamsStruct;

mod hmi;
use crate::hmi::hmi;

extern crate modulos_comunes;
use modulos_comunes::{DataStruct, TcpMessage, EMPTYTCPMESSAGE};

mod procesamiento;
use crate::procesamiento::proc;

const RUEDA_X: f32 = 0.0; 
const RUEDA_Y: f32 = 600.0; 
const CAMA_X: f32 = 300.0; 
const CAMA_Y: f32 = 0.0; 
const CAMA_DESP: f32 = 100.0; 
const LED_X: f32 = 1200.0; 
const LED_Y: f32 = 100.0; 
const SELECTOR_X: f32 = 1500.0; 
const SELECTOR_Y: f32 = 300.0; 

#[macroquad::main("Pogos")]
async fn main() {
    match TcpStream::connect("127.0.0.1:3333") {
        Ok(mut stream) => {
            stream.set_nonblocking(true).expect("set_nonblocking failed");
            println!("Successfully connected to server in port 3333");

            let cinta = quad_gif::GifAnimation::load("/home/mirko/UNRN/3ro/2do_cuatrimestre/Instrumentacion/Pogopins/pogopins_cerradura/cliente/images/Transportadora-2.gif".to_string()).await;
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
            let mut params: [ParamsStruct; 4] = [
                ParamsStruct {
                    foto: led_encendido,
                    gif: Some(cinta),
                    rot: 0.0,
                    animada: false,
                    x: RUEDA_X,
                    y: RUEDA_Y,
                },
                ParamsStruct {
                    foto: cama,
                    gif: None,
                    rot: 0.0,
                    animada: false,
                    x: CAMA_X,
                    y: CAMA_Y,
                },
                ParamsStruct {
                    foto: led_apagado,
                    gif: None,
                    rot: 0.0,
                    animada: false,
                    x: LED_X,
                    y: LED_Y,
                },
                ParamsStruct {
                    foto: selector,
                    gif: None,
                    rot: 0.0,
                    animada: false,
                    x: SELECTOR_X,
                    y: SELECTOR_Y,
                },
            ];

            let mut sensores = DataStruct {
                cinta: false,
                pogos: false,
                selector: false,
                sensor: false,
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
                        println!("Llegó {}", from_utf8(&data).unwrap());
                        match from_utf8(&data).unwrap() {
                            "h" => {
                                sensores.cinta ^= true; 
                            }
                            "j" => {
                                sensores.pogos ^= true; 
                            }
                            "k" => {
                                sensores.sensor ^= true;
                            }
                            "l" => {
                                sensores.selector ^= true;
                            }
                            _ => {}
                        }
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
