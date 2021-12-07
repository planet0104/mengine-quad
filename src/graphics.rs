use macroquad::prelude::*;

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
    image: Texture2D,
    src: Option<[f32; 4]>,
    dest: Option<[f32; 4]>,
) {

    if src.is_some() || dest.is_some(){
        let mut params = DrawTextureParams::default();

        if let Some(src) = src{
            params.source.replace(Rect::new(src[0], src[1], src[2], src[3]));
        }
        let mut x = 0.;
        let mut y = 0.;
        if let Some(dest) = dest{
            x = dest[0];
            y = dest[1];
            params.dest_size = Some(vec2(dest[2], dest[3]));
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

pub fn draw_text(cotnent: &str, x: f32, y: f32, color: &[u8; 4], font_size: f32) {
    macroquad::prelude::draw_text(cotnent, x, y, font_size, Color::from_rgba(color[0], color[1], color[2], color[3]));
}