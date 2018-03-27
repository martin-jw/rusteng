extern crate glutin;
extern crate gl;

use glutin::GlContext;

pub struct RenderWindow {
	gl_window: glutin::GlWindow
}

impl RenderWindow {

	pub fn create(window: glutin::WindowBuilder, events_loop: &glutin::EventsLoop) -> RenderWindow {

		let context = glutin::ContextBuilder::new();

		RenderWindow {
			gl_window: glutin::GlWindow::new(window, context, events_loop).unwrap()
		}
	}

	pub fn init(&self) {
		unsafe {
			self.gl_window.make_current().unwrap();
		}

		unsafe {
			gl::load_with(|symbol| self.gl_window.get_proc_address(symbol) as *const _);
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
		}
	}

	pub fn render(&mut self, _delta: f64) {
		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}

		self.gl_window.swap_buffers().unwrap();
	}
}