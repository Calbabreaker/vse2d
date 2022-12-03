struct State {}

impl vse2d::EventHandler for State {
    fn update(&mut self, ctx: &mut vse2d::Context) {
        if ctx.input.key_just_pressed(vse2d::Keycode::Escape) {
            println!("quit");
            ctx.request_quit();
        }
    }

    fn render(&self, ctx: &mut vse2d::Context) {
        ctx.render.begin(vse2d::Color::from_hex(0x001111));
        println!("{}", ctx.input.mouse_position());
        ctx.render.quad(
            ctx.input.mouse_position(),
            vse2d::vec2(100., 100.),
            vse2d::Color::from_hex(0xff0000),
        );
    }
}

fn main() {
    let context = vse2d::ContextBuilder::new("Simple Game!")
        .resizable(true)
        .build();
    vse2d::run(context, State {}).unwrap_or_else(|err| println!("{err}"));
}
