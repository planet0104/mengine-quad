use aliens::Timmy;
use mengine_quad::{run, engine::{Resource, GameEngine, Sprite, BA_WRAP}, Animation, State, Settings, Event, Drawable};
use macroquad::{prelude::*, audio::{load_sound, Sound, self}};
use anyhow::Result;

mod aliens;

pub const CLIENT_WIDTH:f32 = 600.0;
pub const CLIENT_HEIGHT:f32 = 450.0;

pub struct Game{
    sound_explode_missile: Sound,
    texture_sm_explosion: Drawable,
    sprites: Vec<Sprite>,
}

impl State for Game{
    fn event(&mut self, _event: Event) {
        
    }
    fn update(&mut self) {
        self.update_sprites();
    }
    fn draw(&mut self){
        self.draw_sprites();
    }
}

impl GameEngine for Game{
    fn sprites_mut(&mut self) -> &mut Vec<Sprite> {
        &mut self.sprites
    }

    fn sprites(&self) -> &Vec<Sprite> {
        &self.sprites
    }

    fn sprite_dying(&mut self, sprite_dying_id: usize) {
        //检查是否子弹精灵死亡
        if self.sprites[sprite_dying_id].name() == "missile"{

            audio::play_sound_once(self.sound_explode_missile);

            //在子弹位置创建一个小的爆炸精灵
            let mut frames = vec![];
            for y in (0..136).step_by(17) {
                frames.push(Rect::new(0., y as f32, 17., 17.));
            }
            let anim = Animation::active(self.texture_sm_explosion, frames, 25.0);

            let mut sprite = Sprite::from_bitmap(
                String::from("sm_explosion"),
                Resource::Animation(anim),
                Rect::new(0.0, 0.0, CLIENT_WIDTH, CLIENT_HEIGHT),
            );
            {
                let dpos = self.sprites[sprite_dying_id].position();
                sprite.set_position(dpos.left(), dpos.top());
            }
            self.add_sprite(sprite);
        }
    }

    fn sprite_collision(&mut self, _hitter_id: usize, _hittee_id: usize) -> bool {
        false
    }
}

#[macroquad::main("Aliens")]
async fn main() -> Result<()> {
    
    clear_background(WHITE);

    // 加载素材
    draw_text("Loading...", screen_width()/2., screen_height()/2., 20., BLACK);

    let texture_missile = Drawable::Texture2D(Texture2D::from_image(&load_image("static/TMissile.png").await?));
    let texture_timmy = Drawable::Texture2D(Texture2D::from_image(&load_image("static/Timmy.png").await?));
    let texture_sm_explosion = Drawable::Texture2D(Texture2D::from_image(&load_image("static/SmExplosion.png").await?));
    let sound_missile = load_sound("static/TMissile.ogg").await?;
    let sound_explode_missile = load_sound("static/SmExplode.ogg").await?;
    
    let timmy_ext = Timmy{ missile: texture_missile, sound_missile };

    let mut frames = vec![];
    for y in (0..136).step_by(17) {
        frames.push(Rect::new(0., y as f32, 33., 17.));
    }
    
    let bounds = Rect::new(0.0, 0.0, CLIENT_WIDTH, 410.0);

    let mut anim =
        Animation::active(texture_timmy, frames, 25.0);
    anim.set_repeat(true);
    
    let mut alien = Sprite::with_bounds_action(
        String::from("timmy"),
        Resource::Animation(anim),
        bounds,
        BA_WRAP,
    );
    alien.set_velocity(3., 0.);
    alien.ext(timmy_ext);
    
    let mut game = Game{ sprites: vec![alien], sound_explode_missile, texture_sm_explosion };
    
    run(&mut game, CLIENT_WIDTH, CLIENT_HEIGHT, Settings{
        background_color: Some(BLACK),
        auto_scale: true,
        show_ups_fps: true,
        ..Default::default()
    }).await;

    Ok(())
}
