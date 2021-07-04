use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        //cls is a common abbreviatrion for "clear the screen". In this case
        //we are telling out context that it should clear the virtual terminal.
        ctx.cls();
        // print "Hello Rust World" at the location (1,1).
        ctx.print(1, 1, "Hello Rust Wold");
    }
}

// POD - short for "plain old data"
// struct Position {
//     x: i32,
//     y: i32,
// }

// impl Component for Position {
//     type Storage = VecStorage<Self>;
// }

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorials")
        .build()?;
    //gs - game state
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    rltk::main_loop(context, gs)
}
