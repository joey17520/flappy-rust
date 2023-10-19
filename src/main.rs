use bracket_lib::prelude::*;

mod game_state;
mod game_player;
mod game_obstacle;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, game_state::State::new())
}
