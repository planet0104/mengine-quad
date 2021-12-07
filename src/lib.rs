pub mod graphics;
pub mod engine;
use graphics::{Transform, draw_text};
use macroquad::{prelude::{Texture2D, rand, Rect, screen_width, screen_height, Color, Vec2, BLACK, draw_rectangle, vec2, get_fps, next_frame, KeyCode}, miniquad::date, camera::{set_camera, set_default_camera, Camera2D}};

#[derive(Debug)]
pub enum Event {
    MouseMove(f32, f32),
    Click(f32, f32),
    KeyDown(KeyCode),
    KeyUp(KeyCode),
}

//计时器
#[derive(Clone)]
pub struct AnimationTimer {
    frame_time: f64,
    next_time: f64,
}

impl AnimationTimer {
    pub fn new(fps: f64) -> AnimationTimer {
        AnimationTimer {
            frame_time: 1000.0 / fps,
            next_time: current_timestamp(),
        }
    }

    pub fn set_fps(&mut self, fps: f64) {
        self.frame_time = 1000.0 / fps;
    }

    pub fn reset(&mut self) {
        self.next_time = current_timestamp();
    }

    pub fn ready_for_next_frame(&mut self) -> bool {
        let now = current_timestamp();
        if now >= self.next_time {
            //更新时间
            self.next_time += self.frame_time;
            true
        } else {
            false
        }
    }
}

pub struct SubImage {
    image: Texture2D,
    region: [f32; 4],
}

impl SubImage {
    pub fn new(image: Texture2D, region: [f32; 4]) -> SubImage {
        SubImage { image, region }
    }

    pub fn draw(&self, transform: Option<Transform>, dest: [f32; 4]) {
        graphics::draw_image(transform, self.image, Some(self.region), Some(dest));
    }
}

#[derive(Clone)]
pub struct Animation {
    timer: AnimationTimer,
    image: Texture2D,
    frames: Vec<[f32; 4]>,
    current: i32,
    repeat: bool,
    active: bool,
    end: bool, //current == frames.len()
    pub position: Option<[f32; 4]>,
}

impl Animation {
    pub fn new(image: Texture2D, frames: Vec<[f32; 4]>, fps: f64) -> Animation {
        Animation {
            timer: AnimationTimer::new(fps),
            image,
            frames,
            current: -1,
            repeat: false,
            active: false,
            end: false,
            position: None,
        }
    }

    pub fn active(image: Texture2D, frames: Vec<[f32; 4]>, fps: f64) -> Animation {
        let mut anim = Self::new(image, frames, fps);
        anim.start();
        anim
    }

    pub fn set_current_frame(&mut self, frame: usize) -> bool{
        if frame < self.frames.len(){
            self.current = frame as i32;
            true
        }else{
            false
        }
    }

    pub fn current_frame(&self) -> usize{
        if self.current <= 0{
            0
        }else{
            self.current as usize
        }
    }

    pub fn frame_width(&self) -> f32 {
        if self.frames.len() == 0 {
            0.0
        } else {
            self.frames[0][2]
        }
    }

    pub fn frame_height(&self) -> f32 {
        if self.frames.len() == 0 {
            0.0
        } else {
            self.frames[0][3]
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_repeat(&mut self, repeat: bool) {
        self.repeat = repeat;
    }

    pub fn is_repeat(&mut self) -> bool {
        self.repeat
    }

    pub fn start(&mut self) {
        self.active = true;
        self.current = -1;
        self.timer.reset();
    }

    pub fn stop(&mut self) {
        self.active = false;
    }

    pub fn is_end(&self) -> bool {
        self.current == self.frames.len() as i32
    }

    /// Tick the animation forward by one step
    pub fn update(&mut self) -> bool {
        let mut jump = false;
        if self.active {
            if self.timer.ready_for_next_frame() {
                self.current += 1;
                if self.current == self.frames.len() as i32 {
                    if self.repeat {
                        self.current = 0;
                    } else {
                        self.active = false;
                    }
                }
                jump = true;
            }
        }
        jump
    }

    pub fn draw(&self, transform: Option<Transform>, dest: [f32; 4]) {
        let mut current = 0;
        if self.current > 0 {
            current = if self.current == self.frames.len() as i32 {
                self.frames.len() as i32 - 1
            } else {
                self.current
            };
        }
        // println!("anim draw current={}", current);
        graphics::draw_image(
            transform,
            self.image,
            Some(self.frames[current as usize]),
            Some(dest),
        );
    }
}

// #[derive(Clone, Copy)]
// pub struct Rect {
//     pub left: f32,
//     pub top: f32,
//     pub right: f32,
//     pub bottom: f32,
// }

// impl Rect {
//     pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Rect {
//         Rect {
//             left: left,
//             top: top,
//             right: right,
//             bottom: bottom,
//         }
//     }

//     pub fn zero() -> Rect {
//         Rect {
//             left: 0.0,
//             top: 0.0,
//             right: 0.0,
//             bottom: 0.0,
//         }
//     }

//     /** 修改rect大小 */
//     pub fn inflate(&mut self, dx: f32, dy: f32) {
//         self.left -= dx;
//         self.right += dx;
//         self.top -= dy;
//         self.bottom += dy;
//     }

//     pub fn offset(&mut self, dx: f32, dy: f32) {
//         self.left += dx;
//         self.right += dx;
//         self.top += dy;
//         self.bottom += dy;
//     }

//     pub fn contain(&self, x: f32, y: f32) -> bool {
//         x >= self.left && x <= self.right && y >= self.top && y <= self.bottom
//     }
// }

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

// use std::cmp::PartialOrd;
// use std::ops::{Add, AddAssign, Sub, SubAssign};

// #[derive(Clone, Debug)]
// pub struct Rect<T: PartialOrd + Add + Sub + AddAssign + SubAssign + Copy + Default> {
//     pub pos: Point<T>,
//     pub size: Size<T>,
// }

// impl<
//         T: PartialOrd + Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign + Copy + Default,
//     > Default for Rect<T>
// {
//     fn default() -> Self {
//         Rect {
//             pos: Point::default(),
//             size: Size::default(),
//         }
//     }
// }

// impl<
//         T: PartialOrd + Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign + Copy + Default,
//     > Rect<T>
// {
//     pub fn new(x: T, y: T, width: T, height: T) -> Rect<T> {
//         Rect {
//             pos: Point::new(x, y),
//             size: Size::new(width, height),
//         }
//     }

//     pub fn left(&self) -> T {
//         self.pos.x
//     }

//     pub fn top(&self) -> T {
//         self.pos.y
//     }

//     pub fn right(&self) -> T {
//         self.pos.x + self.size.width
//     }

//     pub fn bottom(&self) -> T {
//         self.pos.y + self.size.height
//     }

//     pub fn width(&self) -> T {
//         self.size.width
//     }

//     pub fn height(&self) -> T {
//         self.size.height
//     }

//     pub fn inflate(&mut self, dx: T, dy: T) {
//         self.pos.x -= dx;
//         self.size.width += dx + dx;
//         self.pos.y -= dy;
//         self.size.height += dy + dy;
//     }

//     pub fn offset(&mut self, dx: T, dy: T) {
//         self.pos.x -= dx;
//         self.pos.y -= dy;
//     }

//     pub fn move_to(&mut self, x: T, y: T) {
//         self.pos.x = x;
//         self.pos.y = y;
//     }

//     pub fn contain(&self, x: T, y: T) -> bool {
//         x >= self.pos.x && x <= self.right() && y >= self.pos.y && y <= self.bottom()
//     }

//     pub fn to_slice(&self) -> [T; 4] {
//         [self.pos.x, self.pos.y, self.size.width, self.size.height]
//     }
// }

// #[derive(Clone, Debug, Copy)]
// pub struct Point<T: Default> {
//     pub x: T,
//     pub y: T,
// }

// impl<T: Default> Point<T> {
//     pub fn new(x: T, y: T) -> Point<T> {
//         Point { x, y }
//     }
// }

// impl<T: Default> Default for Point<T> {
//     fn default() -> Self {
//         Point {
//             x: T::default(),
//             y: T::default(),
//         }
//     }
// }

#[derive(Clone, Debug, Copy)]
pub struct Size<T: Default> {
    pub width: T,
    pub height: T,
}

impl<T: Default> Default for Size<T> {
    fn default() -> Self {
        Size {
            width: T::default(),
            height: T::default(),
        }
    }
}

impl<T: Default> Size<T> {
    pub fn new(width: T, height: T) -> Size<T> {
        Size { width, height }
    }
}

///A builder that constructs a Window
#[derive(Debug)]
pub struct Settings {
    /// If the cursor should be visible over the application
    pub show_cursor: bool,
    /// The smallest size the user can resize the window to
    ///
    /// Does nothing on web
    pub min_size: Option<Size<f32>>,
    /// The largest size the user can resize the window to
    ///
    /// Does nothing on web
    pub max_size: Option<Size<f32>>,
    pub fullscreen: bool,
    /// How many times is the update method called per second
    pub ups: u64,
    pub icon_path: Option<&'static str>, // TODO: statiC?
    /// 背景色[r,g,b,a]
    pub background_color: Option<Color>,
    pub window_size: Option<Vec2>,
    /// 居中绘图
    pub draw_center: bool,
    /// 自动缩放
    pub auto_scale: bool,
    /// 显示更新频率 UPS/FPS
    pub show_ups_fps: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            show_cursor: true,
            min_size: None,
            max_size: None,
            fullscreen: false,
            ups: 60,
            icon_path: None,
            background_color: None,
            draw_center: true,
            auto_scale: false,
            window_size: None,
            show_ups_fps: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AudioType {
    WAV,
    MP3,
    OGG,
    FLAC,
    Other,
}

impl AudioType {
    pub fn test(path: &str) -> AudioType {
        let path = path.to_ascii_lowercase();
        if path.ends_with("wav") {
            AudioType::WAV
        } else if path.ends_with("mp3") {
            AudioType::MP3
        } else if path.ends_with("ogg") {
            AudioType::OGG
        } else if path.ends_with("flac") {
            AudioType::FLAC
        } else {
            AudioType::Other
        }
    }
}

// pub struct AssetsFile {
//     file_name: String,
//     data: Option<Vec<u8>>,
//     tmp_data: Arc<Mutex<Option<Vec<u8>>>>,
// }

// impl AssetsFile {
//     pub fn new(file_name: &str) -> AssetsFile {
//         AssetsFile {
//             file_name: file_name.to_string(),
//             data: None,
//             tmp_data: Arc::new(Mutex::new(None)),
//         }
//     }

//     pub fn load(&mut self) {
//         #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
//         {

//         }

//         #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
//         {
//
//         }
//     }

//     pub fn data(&mut self) -> Option<&Vec<u8>> {
//         if self.data == None {
//             if let Ok(mut tmp_data) = self.tmp_data.try_lock() {
//                 if tmp_data.is_some() {
//                     //移出加载的临时文件数据
//                     self.data = tmp_data.take();
//                 }
//             }
//         }
//         self.data.as_ref()
//     }
// }

pub fn random() -> f64{
    rand::gen_range(0., 1.)
}

//生成指定范围的随即整数
pub fn rand_int(l: i32, b: i32) -> i32 {
    rand::gen_range(l, b)
}

/// 膨胀
pub fn inflate(rect: &Rect, dx: f32, dy: f32) -> Rect {
    Rect::new(rect.x - dx, rect.y - dy, rect.w + dx + dx, rect.h + dy + dy)
}

/// 系统时间戳ms
pub fn current_timestamp() -> f64{
    date::now() * 1000.
}

pub async fn run<T: State>(state: &mut T, width: f32, height: f32, settings: Settings) {

    let background_color = settings.background_color.unwrap_or(BLACK);
    let (auto_scale, draw_center) = (settings.auto_scale, settings.draw_center);
    
    loop {
        state.update();
        let (window_width, window_height) = (screen_width(), screen_height());

        // draw_rectangle(0., 0., window_width, window_height, background_color);
        
        let (mut new_width, mut new_height) = (width, height);
        let (mut scale_x, mut scale_y) = (1.0, 1.0);
        if auto_scale {
            //画面不超过窗口高度
            new_height = window_height;
            new_width = new_height / height * width;

            if new_width > window_width {
                new_width = window_width;
                new_height = new_width / width * height;
            }
            scale_x = new_width / width;
            scale_y = new_height / height;
        }
        let (mut trans_x, mut trans_y) = (0.0, 0.0);
        if draw_center {
            trans_x = (window_width - new_width) / 2.;
            trans_y = (window_height - new_height) / 2.;
        }

        // println!("{}x{} scale={}x{}", window_width, window_height, scale_x, scale_y);

        // println!("{}x{}", trans_x, trans_y);
        let camera_zoom_x = 1./window_width*2.;
        let camera_zoom_y = -1./window_height*2.;
        set_camera(&Camera2D {
            zoom: vec2( camera_zoom_x * scale_x,  camera_zoom_y * scale_y),
            offset: vec2(-1. + camera_zoom_x*trans_x, 1. + camera_zoom_y*trans_y),
            ..Default::default()
        });
        
        state.draw();

        // draw_rectangle(0., 0., width/2., height/2., GRAY);

        //显示UPS/FPS
        if settings.show_ups_fps {
            let _ = draw_text(
                &format!("FPS:{}", get_fps(),),
                20.,
                height - 30.,
                &[255, 255, 0, 200],
                10.,
            );
        }

        set_default_camera();
        
        //遮盖上部分窗口
        draw_rectangle(0., 0., window_width, trans_y, background_color);
        //遮盖下部分窗口
        draw_rectangle(
            0.,
            trans_y + height * scale_y,
            window_width,
            window_height - (trans_y + height * scale_y),
            background_color
        );
        //遮盖左部分窗口
        draw_rectangle(0.0, 0.0, trans_x, window_height, background_color);
        //遮盖右部分窗口
        draw_rectangle(
            trans_x + width * scale_x,
            0.0,
            window_width - (trans_x + width * scale_x),
            window_height,
            background_color
        );
        
        next_frame().await
    }
}

pub trait State{
    fn event(&mut self, _event: Event);
    fn update(&mut self);
    fn draw(&mut self);
}