use std::time::Duration;
use vse2d::KeyCode;

#[derive(Default)]
struct State {
    position: vse2d::Vec2,
    last_check_fps: Duration,
}

impl vse2d::EventHandler for State {
    fn update(&mut self, ctx: &mut vse2d::Context) {
        if ctx.time.now() - self.last_check_fps > Duration::from_secs(1) {
            println!("Frame rate: {:2}", ctx.time.frame_rate());
            self.last_check_fps = ctx.time.now()
        }

        if ctx.input.key_just_pressed(KeyCode::Escape) {
            println!("quit");
            ctx.request_quit();
        }

        let dir =
            ctx.input
                .key_direction(KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down);
        self.position += dir.normalize_or_zero() * ctx.time.delta().as_secs_f32() * 500.;
    }

    fn render(&self, ctx: &mut vse2d::Context) {
        ctx.render.begin(vse2d::Color::from_hex(0x001111));
        ctx.render.quad(
            self.position,
            vse2d::vec2(100., 100.),
            vse2d::Color::from_hex(0xff0000),
        );
    }
}

fn main() {
    let context = vse2d::ContextBuilder::new("Simple Game")
        .resizable(true)
        .build();
    vse2d::run(context, State::default()).unwrap_or_else(|err| println!("{err}"));
}
