use raylib::prelude::*;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct ScrollBlock {
    pub rect: Rectangle,
    screen_width: f32,
    screen_height: f32,
    up_color: Color,
    up_color_bg: Color,
    down_color: Color,
    down_color_bg: Color,

    scroll_size: f32,
    step_size: f32,

    going_up: bool,
    pub last_scrolled_at: f64,
    pub last_scrolled: f64,
    pub scrolls_in_a_row: u16,
    pub max_scrolls: u16,
}

impl ScrollBlock {
    pub fn new(
        width: f32,
        height: f32,
        initial_x: f32,
        initial_y: f32,
        screen_width: f32,
        screen_height: f32,
        up_color: Color,
        down_color: Color,
    ) -> Self {
        let multiplier = 0.2;
        ScrollBlock {
            rect: Rectangle {
                x: initial_x,
                y: initial_y,
                width,
                height,
            },
            screen_width,
            screen_height,
            up_color,
            up_color_bg: multiply_color(up_color, multiplier),
            down_color_bg: multiply_color(down_color, multiplier),
            down_color,
            step_size: width / 2.0,
            scroll_size: height,
            going_up: true,

            last_scrolled_at: 0.0,
            last_scrolled: 0.0,
            scrolls_in_a_row: 0,
            max_scrolls: 0,
        }
    }

    pub fn scroll(&mut self, amount: i64, time: f64) {
        self.rect.y += self.scroll_size * amount as f32;

        // if there is a change in scroll direction
        if (amount < 0) != self.going_up {
            self.going_up = amount < 0;
            // reset scrolls_in_a_row
            self.scrolls_in_a_row = 0;
        }
        // checks to ensure that we arent overstepping any boundaries
        if self.rect.y < 0.0 {
            self.rect.y += self.screen_height;
        } else if self.rect.y >= self.screen_height {
            self.rect.y -= self.screen_height;
        }

        // set last scrolled
        let max_scroll_time = 0.1;
        if (self.last_scrolled >= max_scroll_time) {
            self.scrolls_in_a_row = 0;
        }

        self.last_scrolled = time - self.last_scrolled_at;
        self.last_scrolled_at = time;
        self.scrolls_in_a_row += amount.abs() as u16;

        if self.max_scrolls < self.scrolls_in_a_row {
            self.max_scrolls = self.scrolls_in_a_row;
        }
    }

    pub fn tick(&mut self, delta: f32) {
        // self.rect.x += self.step_size;
        self.rect.x += 400.0 * delta;

        if self.rect.x < 0.0 {
            self.rect.x += self.screen_width;
        } else if self.rect.x >= self.screen_width {
            self.rect.x -= self.screen_width;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rec(self.rect, self.get_color());
    }

    pub fn draw_bg(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rec(self.rect, self.get_color_bg());
    }

    pub fn get_color(&self) -> Color {
        if self.going_up {
            self.up_color
        } else {
            self.down_color
        }
    }

    pub fn get_color_bg(&self) -> Color {
        if self.going_up {
            self.up_color_bg
        } else {
            self.down_color_bg
        }
    }

    pub fn get_parallelogram(&self) -> (Triangle, Triangle) {
        let middle_top = Vector2 {
            x: self.rect.x + self.rect.width / 2.0,
            y: self.rect.y,
        };

        let middle_bottom = Vector2 {
            x: self.rect.x + self.rect.width / 2.0,
            y: self.rect.y + self.rect.height,
        };

        let left_triangle = Triangle {
            v1: middle_top,
            v2: Vector2 {
                x: self.rect.x,
                y: self.rect.y + self.rect.height,
            },
            v3: middle_bottom,
        };

        let right_triangle = Triangle {
            v1: middle_top,
            v2: middle_bottom,
            v3: Vector2 {
                x: self.rect.x + self.rect.width,
                y: self.rect.y,
            },
        };
        (left_triangle, right_triangle)
    }
}

pub fn multiply_color(color: Color, multiplier: f32) -> Color {
    Color {
        r: (color.r as f32 * multiplier) as u8,
        g: (color.g as f32 * multiplier) as u8,
        b: (color.b as f32 * multiplier) as u8,
        a: color.a,
    }
}

pub struct Triangle {
    pub v1: Vector2,
    pub v2: Vector2,
    pub v3: Vector2,
}

impl Triangle {
    pub fn new() -> Self {
        let zero = Vector2::zero();
        Triangle {
            v1: zero.clone(),
            v2: zero.clone(),
            v3: zero,
        }
    }
}
