pub mod game_state;

use bracket_lib::prelude::*;
use crate::game_state::states;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, states::State::new())
}
