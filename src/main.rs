extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use std::time;

mod event;
mod game;
mod entity;
mod ball;
mod ai;
mod util;

pub fn main() {

    /* setup sdl2 */
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("pong", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    /* clear screen and setup auto resizing */
    let mut renderer = window.renderer().build().unwrap();
    let _ = renderer.set_logical_size(800, 600);
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut gamestate = game::State::new();

    /* get events */
    'game: loop {
        let event_list = event::get_event(&mut event_pump);
        for event in event_list {
            match event {
                Event::Quit {..} => { break 'game; },

                Event::KeyDown {keycode: Some(keycode), ..} => {
                   gamestate.input(keycode);
                },

                _ => {}
            }
        }

        gamestate.update();
        renderer.clear();
        gamestate.render(&mut renderer);
        renderer.present();

        /* don't actually do this, use a deltatime instead */
        std::thread::sleep(time::Duration::from_millis(16));
    }
}
