mod level;

use level::state::{Map, CaseState, Direction, State};



pub fn main() -> Result<(), String> {
    let mut state = State {
        map: Map::<CaseState> {
            height: 10,
            width: 10,
            map: vec![CaseState::Empty; 100],
        },
        spots: Vec::new(),
        mario_x: 0,
        mario_y: 0,
        mario_orientation: Direction::Down,
    };

    *state.map.get_mut(1, 1).unwrap() = CaseState::Box;
    state.spots.push((1, 2));

    level::Level::new(state).run()
}
