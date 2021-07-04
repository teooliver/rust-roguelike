use rltk::{GameState, Rltk};

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        //cls is a common abbreviatrion for "clear the screen". In this case
        //we are telling out context that it should clear the virtual terminal.
        ctx.cls();
        // print "Hello Rust World" at the location (1,1).
        ctx.print(1, 1, "Hello Rust Wold");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorials")
        .build()?;
    //gs - game state
    let gs = State {};
    rltk::main_loop(context, gs)
}
