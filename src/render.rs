use glow::HasContext;

use crate::{Color, WindowConfig};

pub struct RenderContext {
    gl: glow::Context,
    _gl_context: sdl2::video::GLContext,
    window: sdl2::video::Window,

    quad_vao: glow::VertexArray,
    shader: glow::Program,
    projection: glam::Mat4,
    blank_texture: glow::Texture,
}

impl RenderContext {
    pub unsafe fn new(config: &WindowConfig, video: &sdl2::VideoSubsystem) -> Result<Self, String> {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let mut window_builder = video.window(config.title, config.width, config.height);
        window_builder.opengl().position_centered();

        if config.resizable {
            window_builder.resizable();
        }
        if config.fullscreen {
            window_builder.fullscreen();
        }

        let window = window_builder.build().map_err(|e| e.to_string())?;
        let _gl_context = window.gl_create_context()?;

        unsafe {
            let gl =
                glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);

            let quad_vao = gl.create_vertex_array()?;
            gl.bind_vertex_array(Some(quad_vao));

            let quad_vbo = gl.create_buffer()?;
            let quad_ibo = gl.create_buffer()?;

            #[rustfmt::skip]
            const QUAD_VERTICIES: &[f32] = &[
               -0.5, -0.5, 0., 0., 0.,
               -0.5,  0.5, 0., 0., 1.,
                0.5,  0.5, 0., 1., 1.,
                0.5, -0.5, 0., 1., 0.,
            ];

            #[rustfmt::skip]
            const QUAD_INDICES: &[u32] = &[
                0, 1, 2,
                2, 3, 0
            ];

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(quad_vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                to_u8_slice(QUAD_VERTICIES),
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(quad_ibo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                to_u8_slice(QUAD_INDICES),
                glow::STATIC_DRAW,
            );

            let f32_size = std::mem::size_of::<f32>() as i32;
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 5 * f32_size, 0);

            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 5 * f32_size, 3 * f32_size);

            let shader = compile_shader(
                &gl,
                include_str!("default_vert.glsl"),
                include_str!("default_frag.glsl"),
            )?;

            let blank_texture = gl.create_texture()?;
            gl.bind_texture(glow::TEXTURE_2D, Some(blank_texture));

            const WHITE_DATA: &[u8] = &[255, 255, 255, 255];
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                1,
                1,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(WHITE_DATA),
            );

            let mut render = Self {
                window,
                gl,
                _gl_context,
                quad_vao,
                shader,
                projection: glam::Mat4::IDENTITY,
                blank_texture,
            };
            render.resize();
            Ok(render)
        }
    }

    pub fn begin(&self, clear_color: Color) {
        unsafe {
            self.gl
                .clear_color(clear_color.r, clear_color.g, clear_color.b, clear_color.a);
            self.gl.clear(glow::COLOR_BUFFER_BIT);

            self.gl.bind_vertex_array(Some(self.quad_vao));
            self.gl.use_program(Some(self.shader));

            self.gl.uniform_matrix_4_f32_slice(
                self.gl
                    .get_uniform_location(self.shader, "u_projection")
                    .as_ref(),
                false,
                self.projection.as_ref(),
            );
        }
    }

    pub fn quad(&self, position: glam::Vec2, size: glam::Vec2, color: Color) {
        unsafe {
            let model = glam::Mat4::from_scale_rotation_translation(
                size.extend(1.),
                glam::Quat::IDENTITY,
                position.extend(0.),
            );

            self.gl.uniform_matrix_4_f32_slice(
                self.gl
                    .get_uniform_location(self.shader, "u_model")
                    .as_ref(),
                false,
                model.as_ref(),
            );

            self.gl.uniform_4_f32_slice(
                self.gl
                    .get_uniform_location(self.shader, "u_color")
                    .as_ref(),
                color.as_ref(),
            );

            self.gl
                .bind_texture(glow::TEXTURE_2D, Some(self.blank_texture));
            self.gl
                .draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
        }
    }

    pub fn end(&self) {
        self.window.gl_swap_window();
    }

    pub fn size(&self) -> glam::Vec2 {
        let (width, height) = self.window.size();
        glam::vec2(width as f32, height as f32)
    }

    pub fn resize(&mut self) {
        let (width, height) = self.window.size();
        self.projection = glam::Mat4::orthographic_lh(0., width as f32, height as f32, 0., -1., 1.);
        unsafe { self.gl.viewport(0, 0, width as i32, height as i32) };
    }
}

unsafe fn compile_shader(
    gl: &glow::Context,
    vertex_source: &str,
    fragment_source: &str,
) -> Result<glow::Program, String> {
    let shader_sources = [
        (glow::VERTEX_SHADER, vertex_source),
        (glow::FRAGMENT_SHADER, fragment_source),
    ];

    let program = gl.create_program()?;
    for (kind, source) in shader_sources {
        let shader = gl.create_shader(kind)?;
        gl.shader_source(shader, source);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            return Err(format!(
                "{} shader failed to compile: \n{}",
                match kind {
                    glow::VERTEX_SHADER => "Vertex",
                    glow::FRAGMENT_SHADER => "Fragment",
                    _ => unreachable!(),
                },
                gl.get_shader_info_log(shader)
            ));
        }

        gl.attach_shader(program, shader);
    }

    gl.link_program(program);
    if !gl.get_program_link_status(program) {
        return Err(gl.get_program_info_log(program));
    }

    Ok(program)
}

fn to_u8_slice<'a, T>(data: &'a [T]) -> &'a [u8] {
    unsafe {
        std::slice::from_raw_parts(
            data.as_ptr() as *const u8,
            data.len() * std::mem::size_of::<T>(),
        )
    }
}
