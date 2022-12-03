use crate::context::Context;
use sdl2::event::{Event, WindowEvent};

pub trait EventHandler {
    fn update(&mut self, context: &mut Context);
    fn render(&self, context: &mut Context);
}

pub fn run<S: EventHandler>(context: Result<Context, String>, mut state: S) -> Result<(), String> {
    let mut context = context?;
    let mut event_pump = context.sdl.event_pump()?;
    while !context.requested_quit {
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

        state.update(&mut context);
        state.render(&mut context);
        context.render.end();
    }

    Ok(())
}
