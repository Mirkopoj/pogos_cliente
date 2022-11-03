use macroquad::prelude::*;

use std::io::{Read, Write, ErrorKind};
use std::net::TcpStream;
use std::str::from_utf8;
use std::{thread, time};

use macroquad::prelude::ImageFormat;

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
const RUEDA_DESP: f32 = 1200.0; 
const CAMA_X: f32 = 300.0; 
const CAMA_Y: f32 = 0.0; 
const CAMA_DESP: f32 = 100.0; 
const LED_X: f32 = 1200.0; 
const LED_Y: f32 = 100.0; 
const SELECTOR_X: f32 = 1500.0; 
const SELECTOR_Y: f32 = 300.0; 

#[macroquad::main("Pogos")]
async fn main() {
    match TcpStream::connect("192.168.1.2:3333") {
        Ok(mut stream) => {
            stream.set_nonblocking(true).expect("set_nonblocking failed");
            println!("Successfully connected to server in port 3333");

            let rueda = Texture2D::from_file_with_format(
                include_bytes!("../images/Rueda.png"),
                Some(ImageFormat::Png),
            );
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
                    foto: rueda,
                    rot: 0.0,
                    x: RUEDA_X,
                    y: RUEDA_Y,
                },
                ParamsStruct {
                    foto: cama,
                    rot: 0.0,
                    x: CAMA_X,
                    y: CAMA_Y,
                },
                ParamsStruct {
                    foto: led_apagado,
                    rot: 0.0,
                    x: LED_X,
                    y: LED_Y,
                },
                ParamsStruct {
                    foto: selector,
                    rot: 0.0,
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
                let params_new = proc(sensores, params, led_apagado, led_encendido, f_time);
                let msg = match hmi(params_new) {
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
                params = params_new;
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
