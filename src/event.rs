use crate::context::Context;
use sdl2::event::{Event, WindowEvent};

pub trait EventHandler {
    fn init(&mut self, _context: &mut Context) {}
    fn update(&mut self, context: &mut Context);
    fn render(&self, context: &mut Context);
}

pub fn run<T: EventHandler>(context: Result<Context, String>, mut state: T) -> Result<(), String> {
    let mut context = context?;
    let mut event_pump = context.sdl.event_pump()?;

    state.init(&mut context);
    while !context.requested_quit {
        context.input.update();

        for event in event_pump.poll_iter() {
            context.input.process_event(&event);
            match event {
                Event::Quit { .. } => context.request_quit(),
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(..) => context.render.resize(),
                    _ => (),
                },
                _ => {}
            }
        }

        context.time.update();
        state.update(&mut context);
        state.render(&mut context);
        context.render.end();
        context.time.wait_sync();
    }

    Ok(())
}
