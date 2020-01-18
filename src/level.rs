pub mod state;

use state::{CaseState, Direction, State};

extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
//use sdl2::mouse::MouseButton;
//use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
//use sdl2::video::{Window, WindowContext};
//use std::env;
use std::path::Path;


fn render(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    state: &State,
    textures_map: &enum_map::EnumMap<CaseState, Option<Texture>>,
    textures_mario: &enum_map::EnumMap<Direction, Texture>,
    texture_spot: &Texture,
    texture_box_on_spot: &Texture,
) {
    for x in 0..state.map.width {
        for y in 0..state.map.width {
            if let Some(texture) = textures_map[*state.map.get(x, y).expect("error1")].as_ref() {
                canvas
                    .copy(
                        &texture,
                        None,
                        Some(Rect::new((x as i32) * 34 as i32, (y as i32) * 34, 34, 34)),
                    )
                    .expect("error2");
            }
        }
    }
    for spot in state.spots.iter() {
        if let Some(case) = state.map.get(spot.0, spot.1) {
            let texture = match case {
                CaseState::Wall | CaseState::Empty => texture_spot,
                CaseState::Box => texture_box_on_spot,
            };
            canvas
                .copy(
                    &texture,
                    None,
                    Some(Rect::new(
                        (spot.0 as i32) * 34,
                        (spot.1 as i32) * 34,
                        34,
                        34,
                    )),
                )
                .expect("error2");
        }
    }
    canvas
        .copy(
            &textures_mario[state.mario_orientation],
            None,
            Some(Rect::new(
                (state.mario_x as i32) * 34,
                (state.mario_y as i32) * 34,
                34,
                34,
            )),
        )
        .expect("error2");
}

pub struct Level {
    state: State,
    render: dyn Fn(&State),
}

fn map_direction(keycode: Keycode) -> Option<Direction> {
    match keycode {
        Keycode::Right => Some(Direction::Right),
        Keycode::Left => Some(Direction::Left),
        Keycode::Up => Some(Direction::Up),
        Keycode::Down => Some(Direction::Down),
        _ => None,
    }
}

impl Level {
    pub fn new(state: State) -> Level {
        
        Level {state}
    }
    pub fn run(&mut self) -> Result<(), String> {
        

        update(&self.state);

        'mainloop: loop {
            for event in sdl_context.event_pump()?.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Option::Some(Keycode::Escape),
                        ..
                    } => break 'mainloop,
                    Event::KeyUp { keycode, .. } => {
                        if let Some(keycode) = keycode {
                            if let Some(direction) = map_direction(keycode) {
                                    self.state.move_mario(direction);
                            }
                            /*canvas.clear();
                            render(
                                &mut canvas,
                                &self.state,
                                &textures_map,
                                &textures_mario,
                                &texture_spot,
                                &texture_box_on_spot,
                            );
                            canvas.present();*/
                            update(&self.state);
                        }
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    }
}