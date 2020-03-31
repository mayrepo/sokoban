mod level;

use level::{EndLevel, state::{Map, CaseState, Direction, State}};
use std::fs;

fn load(path: &str) -> Vec<State> {
    let levels = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    
    
    
    levels.split("\n").map(|line| {
        let mut mario_x = 0;
        let mut mario_y = 0;
        let mut spots = vec![];
        let map = line.chars().take(144).enumerate().map(|n| match n {
            (_, '1') => CaseState::Wall,
            (_, '0') => CaseState::Empty,
            (_, '2') => CaseState::Box,
            (i, '3') => {
                spots.push(((i%12) as level::state::Coordinate, (i/12) as level::state::Coordinate));
                CaseState::Empty
            },
            (i, '4') => {
                mario_x = (i%12) as level::state::Coordinate;
                mario_y = (i/12) as level::state::Coordinate;
                CaseState::Empty
            },
            (i, '5') => {
                mario_x = (i%12) as level::state::Coordinate;
                mario_y = (i/12) as level::state::Coordinate;
                CaseState::Wall
            },
            (i, j) => {println!(":::{}::::{}::::", i, j);panic!();}
        }).collect();
        
        State {
            map: Map/*::<CaseState>*/ {
                height: 12,
                width: 12,
                map,
            },
            spots,
            mario_x,
            mario_y,
            mario_orientation: Direction::Down,
        }
    }).collect()
}

pub fn main() -> Result<(), String> {
    let levels = load("assets/niveaux.lvl");/*vec![State {
        map: Map::<CaseState> {
            height: 10,
            width: 10,
            map: vec![CaseState::Empty; 100],
        },
        spots: Vec::new(),
        mario_x: 0,
        mario_y: 0,
        mario_orientation: Direction::Down,
    }];

    *levels[0].map.get_mut(1, 1).unwrap() = CaseState::Box;
    levels[0].spots.push((1, 2));*/
    
    let mut i=0;
    
    while i<levels.len() {
        match level::Level::new(levels[i].clone()).run()? {
            EndLevel::NextLevel => i+=1,
            EndLevel::PreviousLevel => i = if i==0 {0} else {i-1},
            EndLevel::Restart => {},
            EndLevel::Exit => break,
        }
    }
    Ok(())
}
