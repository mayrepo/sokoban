mod level;

use level::state::{Map, CaseState, Direction, State};



pub fn main() -> Result<(), String> {
    let mut levels = vec![State {
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
    levels[0].spots.push((1, 2));
    
    for level in levels {
        level::Level::new(level).run()?
    }
    Ok(())
}
