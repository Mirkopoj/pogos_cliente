use macroquad::prelude::*;
use quad_gif::GifAnimation;

pub struct ParamsStruct {
    pub foto: Texture2D,
    pub gif: Option<GifAnimation>,
    pub rot: f32,
    pub animada: bool,
    pub x: f32,
    pub y: f32,
    pub pivot: Option<Vec2>,
}
