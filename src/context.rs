use crate::{input::InputContext, render::RenderContext};

pub struct WindowConfig {
    pub(crate) title: &'static str,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) resizable: bool,
    pub(crate) fullscreen: bool,
    pub(crate) vsync: bool,
}

pub struct ContextBuilder {
    window_config: WindowConfig,
}

impl ContextBuilder {
    pub fn new(window_title: &'static str) -> Self {
        Self {
            window_config: WindowConfig {
                title: window_title,
                width: 900,
                height: 600,
                resizable: false,
                fullscreen: false,
                vsync: true,
            },
        }
    }

    pub fn resizable(&mut self, resizable: bool) -> &mut Self {
        self.window_config.resizable = resizable;
        self
    }

    pub fn fullscreen(&mut self, fullscreen: bool) -> &mut Self {
        self.window_config.fullscreen = fullscreen;
        self
    }

    pub fn vsync(&mut self, vsync: bool) -> &mut Self {
        self.window_config.vsync = vsync;
        self
    }

    pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_config.width = width;
        self.window_config.height = height;
        self
    }

    pub fn build(&mut self) -> Result<Context, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        Ok(Context {
            render: unsafe { RenderContext::new(&self.window_config, &video)? },
            input: InputContext::default(),
            sdl,
            requested_quit: false,
        })
    }
}

/// Struct containing all the state data and logic of the engine
/// Use ContextBuilder (using builder pattern) to create this
pub struct Context {
    pub(crate) sdl: sdl2::Sdl,
    pub render: RenderContext,
    pub input: InputContext,
    pub(crate) requested_quit: bool,
}

impl Context {
    pub fn request_quit(&mut self) {
        self.requested_quit = true;
    }
}
