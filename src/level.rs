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
    
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
        let window = video_subsystem
            .window(
                "rust-sdl2 demo: Video",
                34 * (self.state.map.width as u32),
                34 * (self.state.map.width as u32),
            )
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        
        let load_texture = |path| texture_creator.load_texture(Path::new(path)).unwrap();
        
        let textures_map = enum_map::enum_map! {
            CaseState::Empty => None,
            CaseState::Box => Some(load_texture("assets/box.jpg")),
            CaseState::Wall => Some(load_texture("assets/wall.jpg")),
        };

        let texture_spot = load_texture("assets/spot.png");
        let texture_box_on_spot = load_texture("assets/box_on_spot.jpg");

        let textures_mario: enum_map::EnumMap<Direction, Texture> = enum_map::enum_map! {
            Direction::Up => load_texture("assets/mario_up.gif"),
            Direction::Down => load_texture("assets/mario_down.gif"),
            Direction::Left => load_texture("assets/mario_left.gif"),
            Direction::Right => load_texture("assets/mario_right.gif"),
        };

        let mut update = move |state: &State| {
            canvas.clear();
            render(
                &mut canvas,
                state,
                &textures_map,
                &textures_mario,
                &texture_spot,
                &texture_box_on_spot,
            );
            canvas.present();
        };
        
        (update)(&self.state);
        
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
                            update(&self.state);
                            if self.state.is_solved() {
                                return Ok(());
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
