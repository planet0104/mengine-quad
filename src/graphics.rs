use macroquad::prelude::*;

use crate::Drawable;

pub struct Transform {
    pub rotate: f32,
    pub translate: (f32, f32),
}
impl Default for Transform {
    fn default() -> Self {
        Transform {
            rotate: 0.0,
            translate: (0.0, 0.0),
        }
    }
}

pub fn draw_image(
    transform: Option<Transform>,
    image: Drawable,
    source: Option<Rect>,
    dest: Option<Rect>,
) {
    match image{
        Drawable::None => (),
        Drawable::Texture2D(image) => {
            if source.is_some() || dest.is_some(){
                let mut params = DrawTextureParams::default();
        
                params.source = source;
                let mut x = 0.;
                let mut y = 0.;
                if let Some(dest) = dest{
                    x = dest.x;
                    y = dest.y;
                    params.dest_size = Some(vec2(dest.w, dest.h));
                }
                if let Some(transform) = transform{
                    if transform.rotate != 0.{
                        params.rotation = transform.rotate;
                    }
                    x += transform.translate.0;
                    y += transform.translate.1; 
                }
                draw_texture_ex(image, x, y, WHITE, params);
            }else{
                draw_texture(image, 0., 0., WHITE);
            }
        }
    }
}

pub fn draw_text(cotnent: &str, x: f32, y: f32, color: &[u8; 4], font_size: f32) {
    macroquad::prelude::draw_text(cotnent, x, y, font_size, Color::from_rgba(color[0], color[1], color[2], color[3]));
}