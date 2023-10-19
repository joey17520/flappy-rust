pub mod states {
    use bracket_lib::prelude::*;

    pub enum GameMode {
        Menu,
        Playing,
        End,
    }

    pub struct State {
        mode: GameMode,
    }

    impl State {
        pub fn new() -> Self {
            State {
                mode: GameMode::Menu,
            }
        }

        pub fn play(&mut self, ctx: &mut BTerm) {
            self.mode = GameMode::Playing;
        }

        pub fn restart(&mut self) {
            self.mode = GameMode::Playing;
        }

        pub fn main_menu(&mut self, ctx: &mut BTerm) {
            ctx.cls();
            ctx.print_centered(5, "Welcome to Flappy Dragon");
            ctx.print_centered(8, "(P) Play Game");
            ctx.print_centered(9, "(Q) Quit Game");

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::P => self.restart(),
                    VirtualKeyCode::Q => ctx.quitting = true,
                    _ => {}
                }
            }
        }

        pub fn dead(&mut self, ctx: &mut BTerm) {
            ctx.cls();
            ctx.print_centered(5, "You're Dead!");
            ctx.print_centered(8, "(P) Play Again");
            ctx.print_centered(9, "(Q) Quit Game");

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::P => self.restart(),
                    VirtualKeyCode::Q => ctx.quitting = true,
                    _ => {}
                }
            }
        }
    }

    impl GameState for State {
        fn tick(&mut self, ctx: &mut BTerm) {
            match self.mode {
                GameMode::Menu => self.main_menu(ctx),
                GameMode::Playing => self.play(ctx),
                GameMode::End => self.dead(ctx),
            }
        }
    }
}