mod scroll_block;
use scroll_block::ScrollBlock;

use rdev::{listen, Event};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

mod util;
use util::Timer;

use raylib::prelude::*;

const C_BG: Color = Color::new(0, 0, 0, 255);

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;

fn main() {
    let (mut rl, rl_thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Scroll tester")
        .build();

    let screen_rec = Rectangle::new(0.0, 0.0, SCREEN_WIDTH, -SCREEN_HEIGHT);
    let position = Vector2::zero();

    // TODO: make frame independent
    // WHAT PART IS FRAME DEPENDENT???
    rl.set_target_fps(500); // fps cap set in order to avoid lagging th game

    let mut scroll_block = ScrollBlock::new(
        20.0,
        40.0,
        SCREEN_WIDTH / 2.0,
        SCREEN_HEIGHT / 2.0,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        Color {
            g: 255,
            ..Color::BLACK
        },
        Color::RED,
    );
    let mut scroll_amount: i32;
    let mut delta;

    // this dictates whether the app takes full focus or not
    let mut cursor_disabled = false;

    // used to track time
    let now = SystemTime::now();

    // this is how often the block will step forward
    // let mut timer = Timer::new(0.02);
    // divide 1 second by 144 frames to get the frametime of 144 fps, in other words once
    // every frame at 144 fps
    let mut texture_clear_timer = Timer::new(1.0 / 144.0);
    println!("Time to tick: {}", texture_clear_timer.time);

    // this is our canvas for trails
    let mut target = rl
        .load_render_texture(&rl_thread, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .unwrap();

    // pre-loop screen clear\
    // apparently useless
    /* {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
    } */

    /* ------------ GLOBALSCROLL ------------ */
    // this part is used to gather mouse scroll information even when window is not in focus
    let scroll_event = Arc::new(Mutex::new(0));
    let scroll_event_clone = Arc::clone(&scroll_event);

    // chat gippity did it
    thread::spawn(move || {
        let scroll_callback = move |event: Event| {
            if let rdev::EventType::Wheel { delta_y, .. } = event.event_type {
                if delta_y != 0 {
                    let mut change = scroll_event_clone.lock().unwrap();
                    // need to dereference before accessing the data
                    *change += delta_y;
                    println!("scrolled: {change}")
                }
            }
        };

        if let Err(error) = listen(scroll_callback) {
            println!("Failed to start scroll input listener.\nError: {:?}", error);
        }
    });

    let mut scroll_diff: i64;

    /* ------------ MAINLOOP ------------ */
    while !rl.window_should_close() {
        {
            let mut scroll = scroll_event.lock().unwrap();
            scroll_amount = scroll.clone() as i32;
            *scroll = 0;
        }
        delta = rl.get_frame_time();
        /* ----------- INPUTS ----------- */
        // WARN: not sure what the output of this function even is so converting to i32 is lazy
        // scroll_amount = rl.get_mouse_wheel_move_v().y as i32;

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            cursor_disabled = !cursor_disabled;
            if cursor_disabled {
                rl.disable_cursor();
            } else {
                rl.enable_cursor();
            }
        }

        /* ------------ LOGIC ----------- */
        if scroll_amount != 0 {
            scroll_block.scroll(scroll_amount, now.elapsed().unwrap().as_secs_f64());
            // debug print
            /* println!(
                "Last scrolled: {}\nScrolls in a row: {}\n\n",
                scroll_block.last_scrolled, scroll_block.scrolls_in_a_row
            ) */
        }

        // move whenever the timer runs out of time
        /* if timer.tick_timeout(delta) {
            timer.soft_reset();
            scroll_block.tick();
        } */

        // move at a constant speed
        scroll_block.tick(delta);

        /* ------------ PAINT ------------ */

        /* ------------ DRAW ------------ */
        let mut d: RaylibDrawHandle = rl.begin_drawing(&rl_thread);

        {
            let mut texture_drawer = d.begin_texture_mode(&rl_thread, &mut target);
            texture_drawer.draw_rectangle_rec(scroll_block.rect, scroll_block.get_color());

            if texture_clear_timer.tick_timeout(delta) {
                texture_drawer.draw_rectangle(
                    0,
                    0,
                    SCREEN_WIDTH as i32,
                    SCREEN_HEIGHT as i32,
                    Color {
                        a: 2,
                        ..Color::BLACK
                    },
                );
                texture_clear_timer.soft_reset();
            }
        }

        d.clear_background(C_BG);
        d.draw_texture_rec(target.texture(), screen_rec, position, Color::WHITE);
        scroll_block.draw(&mut d);

        // draw UI
        d.draw_fps(0, 0);
    }
}

fn callback(event: Event) {
    if let rdev::EventType::Wheel { delta_y, .. } = event.event_type {
        if delta_y != 0 {
            println!("{}", delta_y);
        }
    }
}
