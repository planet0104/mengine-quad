use mengine_quad::{run, engine::{Resource, GameEngine, Sprite, ScrollingBackground, BackgroundLayer, ScrollDir, BA_STOP}, Animation, State, Settings, Event, Drawable, rand_uuid};
use macroquad::{prelude::*};
use anyhow::Result;

pub const CLIENT_WIDTH:f32 = 256.0;
pub const CLIENT_HEIGHT:f32 = 256.0;

pub struct Game{
    font: Font,
    background: ScrollingBackground,
    foreground: ScrollingBackground,
    sprites: Vec<Sprite>,
}

impl Game{
    fn drive(&mut self, direction: ScrollDir){
        //让人走动
        if let Some(player_anim) = self.sprites_mut()[0].resource_mut().amination_mut(){
            if player_anim.current_frame() == 0{
                player_anim.set_current_frame(1);
            }else{
                player_anim.set_current_frame(0);
            }
        }

        //向右移动风景图层
        self.background.layers()[0].set_speed(16.0);
        self.background.layers()[0].set_direction(direction);
        self.background.update();
        self.background.layers()[0].set_speed(0.0);
        
        //向右移动云彩图层
        self.foreground.layers()[0].set_speed(4.0);
        self.foreground.layers()[0].set_direction(direction);
        self.foreground.update();
        self.foreground.layers()[0].set_speed(0.0);
    }
}

impl State for Game{
    fn event(&mut self, _event: Event) {
        
    }
    fn update(&mut self) {
        self.update_sprites();
        // self.background.update();
        // self.foreground.update();
        
        if is_key_pressed(KeyCode::Left){
            self.drive(ScrollDir::Left)
        }else if is_key_pressed(KeyCode::Right){
            self.drive(ScrollDir::Right)
        }else if is_key_pressed(KeyCode::Up){
            self.drive(ScrollDir::Up)
        }else if is_key_pressed(KeyCode::Down){
            self.drive(ScrollDir::Down)
        }
    }
    fn draw(&mut self){
        self.background.draw();
        self.draw_sprites();
        self.foreground.draw();

        let hint = "上下左右键移动";
        let font_size = 16;
        let hint_x = 5.;
        let hint_baseline = 20.;
        let size = measure_text(hint, Some(self.font), font_size, 1.0);
        draw_rectangle(hint_x, hint_baseline-font_size as f32 + 2., size.width, font_size as f32, Color::from_rgba(0, 0, 0, 100));
        draw_text_ex(
            hint,
            hint_x,
            hint_baseline,
            TextParams {
                font_size,
                font: self.font,
                color: WHITE,
                ..Default::default()
            },
        );
    }
}

impl GameEngine for Game{
    fn sprites_mut(&mut self) -> &mut Vec<Sprite> {
        &mut self.sprites
    }

    fn sprites(&self) -> &Vec<Sprite> {
        &self.sprites
    }

    fn sprite_dying(&mut self, _sprite_dying_id: usize) {
        
    }

    fn sprite_collision(&mut self, _hitter_id: usize, _hittee_id: usize) -> bool {
        false
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Aliens".to_owned(),
        window_width: 250,
        window_height: 150,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    
    clear_background(WHITE);

    // 加载素材
    draw_text("Loading...", screen_width()/2., screen_height()/2., 20., BLACK);

    let font = load_ttf_font("static/VonwaonBitmap-16pxLite.ttf").await?;

    let texture_background_clouds = Drawable::Texture2D(Texture2D::from_image(&load_image("static/Background_Clouds.png").await?));
    let texture_background_landscape = Drawable::Texture2D(Texture2D::from_image(&load_image("static/Background_Landscape.png").await?));
    let texture_persion = Drawable::Texture2D(Texture2D::from_image(&load_image("static/Person.png").await?));
    
    //创建滚动背景和风景图层
    let mut background = ScrollingBackground::new();
    let bg_landscape_layer = BackgroundLayer::new(
        texture_background_landscape,
        Rect::new(352.0, 352.0, 608.0, 608.0), //视口最初设置为显示风景位图的中央
        0.0,
        ScrollDir::Left,
    );
    background.add_layer(bg_landscape_layer);


    //创建滚动前景和云彩图层
    let mut foreground = ScrollingBackground::new();
    let fg_clouds_layer = BackgroundLayer::new(
        texture_background_clouds,
        Rect::new(64.0, 64.0, 320.0, 320.0),
        0.0,
        ScrollDir::Left,
    );
    foreground.add_layer(fg_clouds_layer);
    

    let frames = vec![
        Rect::new(0., 0., 26., 32.),
        Rect::new(0., 32., 26., 32.)
    ];

    let mut anim =
        Animation::active(texture_persion, frames, 25.0);
    // anim.set_repeat(true);
    anim.stop();
    
    let mut person = Sprite::with_bounds_action(
        rand_uuid(),
        String::from("timmy"),
        Resource::Animation(anim),
        Rect::new(115.0, 112.0, 26.0, 32.0),
        BA_STOP,
    );
    person.set_position(115.0, 112.0);
    
    let mut game = Game{ sprites: vec![person], foreground, background, font };
    
    run(&mut game, CLIENT_WIDTH, CLIENT_HEIGHT, Settings{
        background_color: Some(WHITE),
        auto_scale: true,
        show_ups_fps: true,
        ..Default::default()
    }).await;

    Ok(())
}
