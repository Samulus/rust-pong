extern crate sdl2;
use sdl2::EventPump;

pub fn get_event(event_pump: &mut EventPump) -> Vec<sdl2::event::Event> {
    let mut output = Vec::new();
    for event in event_pump.poll_iter() {
        output.push(event);
    }
    return output;
}
