use instant::Instant;

use luminance::{context::GraphicsContext, pipeline::PipelineState};

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

#[cfg_attr(wasm, wasm_bindgen)]
pub struct Renderer {
    start_time: Instant,
    surface: Surface,
}

#[cfg_attr(wasm, wasm_bindgen)]
impl Renderer {
    #[cfg(wasm)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Renderer {
        console_error_panic_hook::set_once();

        let surface = {
            let surface = WebSysWebGL2Surface::new("game").expect("web-sys surface");

            surface
        };

        let start_time = Instant::now();

        Renderer {
            start_time,
            surface,
        }
    }

    #[cfg(not(wasm))]
    pub fn new() -> (Renderer, glutin::event_loop::EventLoop<()>) {
        let (surface, event_loop) = {
            let wb = glutin::window::WindowBuilder::new()
                .with_title("Hello Luminance!")
                .with_inner_size(glutin::dpi::LogicalSize::new(400.0, 300.0));

            GlutinSurface::new_gl33(wb, 0).unwrap()
        };

        let start_time = Instant::now();

        (
            Renderer {
                start_time,
                surface,
            },
            event_loop,
        )
    }

    #[cfg_attr(wasm, wasm_bindgen)]
    pub fn render(&mut self) {
        let elapsed = self.start_time.elapsed().as_millis() as f32 * 0.001;
        let color = [elapsed.cos(), elapsed.sin(), 0.5, 1.];
        let back_buffer = self.surface.back_buffer().unwrap();

        let _render = self
            .surface
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default().set_clear_color(color),
                |_, _| Ok(()),
            )
            .assume()
            .into_result()
            .expect("Could not render");

        #[cfg(not(wasm))]
        self.surface.swap_buffers();
    }
}
