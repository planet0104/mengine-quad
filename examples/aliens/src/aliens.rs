use macroquad::{prelude::{Texture2D, Rect}, audio::{self, Sound}};
use mengine_quad::{engine::{SpriteExt, SPRITEACTION, SA_ADDSPRITE, Sprite, Resource, BA_DIE}, rand_int, Point};

//外星人
pub struct Timmy {
    pub sound_missile: Sound,
    pub missile: Texture2D
}

impl SpriteExt for Timmy {
    fn update(&self, sprite_action: SPRITEACTION) -> SPRITEACTION {
        //检查精灵是否要发射子弹
        match rand_int(0, 30) {
            0 => sprite_action | SA_ADDSPRITE,
            _ => sprite_action,
        }
        // sprite_action
    }

    fn add_sprite(&self, sprite: &Sprite) -> Sprite {
        //创建一个新的子弹精灵
        let bounds = Rect::new(0.0, 0.0, 640.0, 410.0);
        let pos = sprite.position();
        let velocity = Point { x: 0.0, y: 3.0 };

        let mut sub_sprite = Sprite::with_bounds_action(
            String::from("missile"),
            Resource::Static(self.missile),
            bounds,
            BA_DIE,
        );
        sub_sprite.set_velocity(velocity.x, velocity.y);

        //播放导弹发射声音
        audio::play_sound_once(self.sound_missile);

        // println!("top={} bottom={}", pos.top(), pos.bottom());
        sub_sprite.set_position(pos.left() + sprite.width() / 2.0, pos.bottom());
        sub_sprite
    }
}