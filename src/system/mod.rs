pub trait System {
	fn update(&mut self, delta: f64);
}

pub trait RenderSystem {
	fn render(&mut self, delta: f64);
}