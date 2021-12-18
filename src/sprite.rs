use sdl2::render::Texture;
use crate::vec2::Vec2;

pub struct Sprite<'a> {
    pub pos: Vec2,
    pub size: Vec2,
    pub tex: &'a Texture<'a>,
}
