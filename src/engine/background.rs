use macroquad::prelude::{Rect, vec2};

use crate::*;

pub struct ScrollingBackground {
    layers: Vec<BackgroundLayer>,
}

impl ScrollingBackground {
    pub fn new() -> ScrollingBackground {
        ScrollingBackground { layers: vec![] }
    }

    pub fn add_layer(&mut self, layer: BackgroundLayer) {
        self.layers.push(layer);
    }

    pub fn draw(&self) {
        for layer in &self.layers {
            layer.draw();
        }
    }

    pub fn update(&mut self) {
        //更新图层
        for layer in &mut self.layers {
            layer.update();
        }
    }

    pub fn layers(&mut self) -> &mut [BackgroundLayer]{
        &mut self.layers
    }
}

#[derive(Clone, Debug, Copy)]
pub enum ScrollDir {
    Up,
    Right,
    Down,
    Left,
}

pub struct BackgroundLayer {
    viewport: Rect,
    speed: f32,
    direction: ScrollDir,
    bitmap: Texture2D,
}

impl BackgroundLayer {
    pub fn new(bitmap: Texture2D, viewport: Rect, speed: f32, direction: ScrollDir) -> BackgroundLayer {
        BackgroundLayer {
            speed,
            direction,
            bitmap,
            viewport,
        }
    }

    pub fn update(&mut self) {
        match self.direction {
            ScrollDir::Up => {
                // Move the layer up (slide the viewport down)
                self.viewport = self.viewport.offset(vec2(0., self.speed));
                if self.viewport.top() > self.height() as f32 {
                    self.viewport.move_to(vec2(self.viewport.left(), 0.));
                }
            }

            ScrollDir::Right => {
                self.viewport = self.viewport.offset(vec2(-self.speed, 0.));
                if self.viewport.right() < 0.0 {
                    self.viewport.move_to(vec2(self.width() as f32 - (self.viewport.right() - self.viewport.left()), self.viewport.top()));
                }
            }

            ScrollDir::Down => {
                self.viewport = self.viewport.offset(vec2(0., -self.speed));
                if self.viewport.bottom() < 0.0 {
                    self.viewport.move_to(vec2(self.viewport.left(), self.height() as f32 - (self.viewport.bottom() - self.viewport.top())))
                }
            }

            ScrollDir::Left => {
                // Move the layer left (slide the viewport right)
                self.viewport = self.viewport.offset(vec2(self.speed, 0.));
                if self.viewport.left() > self.width() as f32 {
                    self.viewport.move_to(vec2(0., self.viewport.top()))
                }
            }
        }
    }

    pub fn draw(&self) {
        let (x, y) = (0.0, 0.0);
        //仅绘制通过视口看到的图层部分
        if self.viewport.top() < 0.0 && self.viewport.left() < 0.0 {
            //绘制分割视口，从上到下，从左到右
            //绘制左上部分(对应图片右下部分)
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.width() as f32 + self.viewport.left(),
                    self.height() as f32 + self.viewport.top(), //图像源左上角
                    -self.viewport.left(),
                    -self.viewport.top(),
                ]), //图像源宽高
                Some([
                    x,
                    y, //目标绘制坐标
                    -self.viewport.left(),
                    -self.viewport.top(),
                ]),
            );
            //绘制右上部分(对应图片左下部分)
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    0.0,
                    self.height() as f32 + self.viewport.top(),
                    -self.viewport.right(),
                    -self.viewport.top(),
                ]),
                Some([
                    x - self.viewport.left(),
                    y,
                    -self.viewport.right(),
                    -self.viewport.top(),
                ]),
            );
            //绘制左下部分(对应图片右上部分)
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.width() as f32 + self.viewport.left(),
                    0.0,
                    -self.viewport.left(),
                    self.viewport.bottom(),
                ]),
                Some([
                    x,
                    y - self.viewport.top(),
                    -self.viewport.left(),
                    self.viewport.bottom(),
                ]),
            );
            //绘制右下部分(对应图片左上部分)
            graphics::draw_image(
                None,
                self.bitmap,
                Some([0.0, 0.0, self.viewport.right(), self.viewport.bottom()]),
                Some([
                    x - self.viewport.left(),
                    y - self.viewport.top(),
                    self.viewport.right(),
                    self.viewport.bottom(),
                ]),
            );
        } else if self.viewport.top() < 0.0 && self.viewport.right() > self.width() as f32 {
            //绘制拆开的视口，从顶部环绕到底部，从右侧环绕到左侧
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.viewport.left(),
                    self.height() as f32 + self.viewport.top(),
                    self.width() as f32 - self.viewport.left(),
                    -self.viewport.top(),
                ]),
                Some([x, y, self.width() as f32 - self.viewport.left(), -self.viewport.top()]),
            );
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    0.0,
                    self.height() as f32 + self.viewport.top(),
                    self.viewport.right() - self.width() as f32,
                    -self.viewport.top(),
                ]),
                Some([
                    x + (self.width() as f32 - self.viewport.left()),
                    y,
                    self.viewport.right() - self.width() as f32,
                    -self.viewport.top(),
                ]),
            );
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.viewport.left(),
                    0.0,
                    self.width() as f32 - self.viewport.left(),
                    self.viewport.bottom(),
                ]),
                Some([
                    x,
                    y - self.viewport.top(),
                    self.width() as f32 - self.viewport.left(),
                    self.viewport.bottom(),
                ]),
            );
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    0.0,
                    0.0,
                    self.viewport.right() - self.width() as f32,
                    self.viewport.bottom(),
                ]),
                Some([
                    x + (self.width() as f32 - self.viewport.left()),
                    y - self.viewport.top(),
                    self.viewport.right() - self.width() as f32,
                    self.viewport.bottom(),
                ]),
            );
        } else if self.viewport.bottom() > self.height() as f32 && self.viewport.left() < 0.0 {
            //绘制拆开的视口，从底部环绕到顶部，从左侧环绕到右侧
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.width() as f32 + self.viewport.left(),
                    self.viewport.top(),
                    -self.viewport.left(),
                    self.height() as f32 - self.viewport.top(),
                ]),
                Some([x, y, -self.viewport.left(), self.height() as f32 - self.viewport.top()]),
            );
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    0.0,
                    self.viewport.top(),
                    self.viewport.right(),
                    self.height() as f32 - self.viewport.top(),
                ]),
                Some([
                    x - self.viewport.left(),
                    y,
                    self.viewport.right(),
                    self.height() as f32 - self.viewport.top(),
                ]),
            );
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.width() as f32 + self.viewport.left(),
                    0.0,
                    -self.viewport.left(),
                    self.viewport.bottom() - self.height() as f32,
                ]),
                Some([
                    x,
                    y + (self.height() as f32 - self.viewport.top()),
                    -self.viewport.left(),
                    self.viewport.bottom() - self.height() as f32,
                ]),
            );
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    0.0,
                    0.0,
                    self.viewport.right(),
                    self.viewport.bottom() - self.height() as f32,
                ]),
                Some([
                    x - self.viewport.left(),
                    y + (self.height() as f32 - self.viewport.top()),
                    self.viewport.right(),
                    self.viewport.bottom() - self.height() as f32,
                ]),
            );
        } else if self.viewport.bottom() > self.height() as f32 && self.viewport.right() > self.width() as f32 {
            //绘制所有窗口，从底部环绕到顶部，从右侧环绕到左侧
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.viewport.left(),
                    self.viewport.top(),
                    self.width() as f32 - self.viewport.left(),
                    self.height() as f32 - self.viewport.top(),
                ]),
                Some([
                    x,
                    y,
                    self.width() as f32 - self.viewport.left(),
                    self.height() as f32 - self.viewport.top(),
                ]),
            );
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    0.0,
                    self.viewport.top(),
                    self.viewport.right() - self.width() as f32,
                    self.height() as f32 - self.viewport.top(),
                ]),
                Some([
                    x + (self.width() as f32 - self.viewport.left()),
                    y,
                    self.viewport.right() - self.width() as f32,
                    self.height() as f32 - self.viewport.top(),
                ]),
            );
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.viewport.left(),
                    0.0,
                    self.width() as f32 - self.viewport.left(),
                    self.viewport.bottom() - self.height() as f32,
                ]),
                Some([
                    x,
                    y + (self.height() as f32 - self.viewport.top()),
                    self.width() as f32 - self.viewport.left(),
                    self.viewport.bottom() - self.height() as f32,
                ]),
            );
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    0.0,
                    0.0,
                    self.viewport.right() - self.width() as f32,
                    self.viewport.bottom() - self.height() as f32,
                ]),
                Some([
                    x + (self.width() as f32 - self.viewport.left()),
                    y + (self.height() as f32 - self.viewport.top()),
                    self.viewport.right() - self.width() as f32,
                    self.viewport.bottom() - self.height() as f32,
                ]),
            );
        } else if self.viewport.top() < 0.0 {
            //绘制拆开的视口，从顶部环绕到底部
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.viewport.left(),
                    self.height() as f32 + self.viewport.top(), //srcx, srcY
                    self.viewport.right() - self.viewport.left(),
                    -self.viewport.top(),
                ]), //width, height
                Some([
                    x,
                    y, //destX, destY
                    self.viewport.right() - self.viewport.left(),
                    -self.viewport.top(),
                ]),
            );
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.viewport.left(),
                    0.0, //srcX, srcY
                    self.viewport.right() - self.viewport.left(),
                    self.viewport.bottom(),
                ]),
                Some([
                    x,
                    y - self.viewport.top(), //destX, destY
                    self.viewport.right() - self.viewport.left(),
                    self.viewport.bottom(),
                ]),
            );
        } else if self.viewport.right() > self.width() as f32 {
            //绘制拆开的视口，从右侧环绕到左侧
            let w = self.width() as f32 - self.viewport.left();
            let h = self.viewport.bottom() - self.viewport.top();
            if w > 0.0 && h > 0.0 {
                graphics::draw_image(
                    None,
                    self.bitmap,
                    Some([self.viewport.left(), self.viewport.top(), w, h]),
                    Some([x, y, w, h]),
                );
            }

            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    0.0,
                    self.viewport.top(),
                    self.viewport.right() - self.width() as f32,
                    self.viewport.bottom() - self.viewport.top(),
                ]),
                Some([
                    x + (self.width() as f32 - self.viewport.left()),
                    y,
                    self.viewport.right() - self.width() as f32,
                    self.viewport.bottom() - self.viewport.top(),
                ]),
            );
        } else if self.viewport.bottom() > self.height() as f32 {
            //绘制拆开的窗口，从底部环绕到顶部
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.viewport.left(),
                    self.viewport.top(),
                    self.viewport.right() - self.viewport.left(),
                    self.height() as f32 - self.viewport.top(),
                ]),
                Some([
                    x,
                    y,
                    self.viewport.right() - self.viewport.left(),
                    self.height() as f32 - self.viewport.top(),
                ]),
            );
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.viewport.left(),
                    0.0,
                    self.viewport.right() - self.viewport.left(),
                    self.viewport.bottom() - self.height() as f32,
                ]),
                Some([
                    x,
                    y + (self.height() as f32 - self.viewport.top()),
                    self.viewport.right() - self.viewport.left(),
                    self.viewport.bottom() - self.height() as f32,
                ]),
            );
        } else if self.viewport.left() < 0.0 {
            //绘制拆开的视口，从左侧环绕到右侧
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.width() as f32 + self.viewport.left(),
                    self.viewport.top(),
                    -self.viewport.left(),
                    self.viewport.bottom() - self.viewport.top(),
                ]),
                Some([
                    x,
                    y,
                    -self.viewport.left(),
                    self.viewport.bottom() - self.viewport.top(),
                ]),
            );
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    0.0,
                    self.viewport.top(),
                    self.viewport.right(),
                    self.viewport.bottom() - self.viewport.top(),
                ]),
                Some([
                    x - self.viewport.left(),
                    y,
                    self.viewport.right(),
                    self.viewport.bottom() - self.viewport.top(),
                ]),
            );
        } else {
            //一次性绘制整个视口
            graphics::draw_image(
                None,
                self.bitmap,
                Some([
                    self.viewport.left(),
                    self.viewport.top(),
                    self.viewport.right() - self.viewport.left(),
                    self.viewport.bottom() - self.viewport.top(),
                ]),
                Some([
                    x,
                    y,
                    self.viewport.right() - self.viewport.left(),
                    self.viewport.bottom() - self.viewport.top(),
                ]),
            );
        }
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn set_direction(&mut self, direction: ScrollDir) {
        self.direction = direction;
    }

    pub fn set_viewport(&mut self, viewport: Rect) {
        self.viewport = viewport;
    }

    pub fn width(&self) -> f32 {
        self.bitmap.width()
    }

    pub fn height(&self) -> f32 {
        self.bitmap.height()
    }
}
