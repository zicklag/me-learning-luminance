use luminance::{
    context::GraphicsContext,
    pipeline::PipelineState,
    render_state::RenderState,
    shader::Program,
    tess::{Mode, Tess},
};
use luminance_derive::*;

use luminance_front::Backend;
#[cfg(not(wasm))]
use luminance_glutin::GlutinSurface;

#[cfg(wasm)]
use luminance_web_sys::WebSysWebGL2Surface;
#[cfg(wasm)]
use wasm_bindgen::prelude::*;

#[cfg(not(wasm))]
type Surface = GlutinSurface;
#[cfg(wasm)]
type Surface = WebSysWebGL2Surface;

compile_error!("Unimplemented 😉");

#[cfg(wasm)]
fn main() {}

#[cfg(not(wasm))]
fn main() {
    let (mut renderer, event_loop) = Renderer::new();

    use glutin::event::{Event, WindowEvent};
    use glutin::event_loop::ControlFlow;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => {
                return;
            }
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }

        renderer.render();
    });
}

#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
    Color,
}

#[derive(Copy, Clone, Debug, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    #[allow(dead_code)]
    position: VertexPosition,

    #[allow(dead_code)]
    #[vertex(normalized = "true")]
    color: VertexRGB,
}

const QUAD_VERTICES: [Vertex; 4] = [
    // Top left
    Vertex::new(
        VertexPosition::new([-0.5, 0.5]),
        VertexRGB::new([255, 0, 0]),
    ),
    // Top right
    Vertex::new(VertexPosition::new([0.5, 0.5]), VertexRGB::new([0, 0, 255])),
    // Bottom right
    Vertex::new(
        VertexPosition::new([0.5, -0.5]),
        VertexRGB::new([0, 255, 0]),
    ),
    // Bottom left
    Vertex::new(
        VertexPosition::new([-0.5, -0.5]),
        VertexRGB::new([0, 0, 255]),
    ),
];

const QUAD_INDICES: [u8; 6] = [0, 1, 2, 0, 2, 3];

#[cfg_attr(wasm, wasm_bindgen)]
pub struct Renderer {
    surface: Surface,
    program: Program<Backend, VertexSemantics, (), ()>,
    quad: Tess<Backend, Vertex, u8>,
}

#[cfg_attr(wasm, wasm_bindgen)]
impl Renderer {
    #[cfg(wasm)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Renderer {
        console_error_panic_hook::set_once();

        let surface = WebSysWebGL2Surface::new("game").expect("web-sys surface");

        init_renderer(surface)
    }

    #[cfg(not(wasm))]
    pub fn new() -> (Renderer, glutin::event_loop::EventLoop<()>) {
        let (surface, event_loop) = {
            let wb = glutin::window::WindowBuilder::new()
                .with_title("Hello Luminance!")
                .with_inner_size(glutin::dpi::LogicalSize::new(400.0, 300.0));

            GlutinSurface::new_gl33(wb, 0).unwrap()
        };

        (init_renderer(surface), event_loop)
    }

    #[cfg_attr(wasm, wasm_bindgen)]
    pub fn render(&mut self) {
        let back_buffer = self.surface.back_buffer().unwrap();

        let Self { program, quad, .. } = self;

        let _render = self
            .surface
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default(),
                |_, mut shading_gate| {
                    shading_gate.shade(program, |_, _, mut render_gate| {
                        render_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(&*quad)
                        })
                    })
                },
            )
            .assume()
            .into_result()
            .expect("Could not render");

        #[cfg(not(wasm))]
        self.surface.swap_buffers();
    }
}

fn init_renderer(mut surface: Surface) -> Renderer {
    let quad = surface
        .new_tess()
        .set_vertices(&QUAD_VERTICES[..])
        .set_indices(&QUAD_INDICES[..])
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();

    let program = surface
        .new_shader_program::<VertexSemantics, (), ()>()
        .from_strings(
            include_str!("hello_texture/shader.vert"),
            None,
            None,
            include_str!("hello_texture/shader.frag"),
        )
        .unwrap()
        .ignore_warnings();

    Renderer {
        surface,
        program,
        quad,
    }
}
