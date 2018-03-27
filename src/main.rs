#![allow(dead_code)]

extern crate glutin;

mod gloop;
mod render;
mod component;
mod resource;
mod system;

#[macro_use]
mod entity;

use glutin::{ Event, WindowEvent };
use gloop::{ GameLoop, LoopState };
use render::{ RenderWindow };
use component::{ Component, Storage };

use std::any::Any;

fn main() {

	let mut gameloop = GameLoop::new();

	let mut events_loop = glutin::EventsLoop::new();
	let window_builder = glutin::WindowBuilder::new().with_title("Rustfun!").with_dimensions(1280, 720);

	let mut window = RenderWindow::create(window_builder, &events_loop);
	window.init();

	let mut manager = entity_manager!{ 50, transform : component::TransformComponent };

	while let Some(state) = gameloop.next() {
		match state {
			LoopState::Input => {
				events_loop.poll_events(|event| {
					match event {
						Event::WindowEvent { event: WindowEvent::Closed, .. } => {
							gameloop.stop();
						}
						_ => {}
					}
				})
			}
			LoopState::Update { delta: _d } => {
			}
			LoopState::Render { delta: _d } => {
				window.render(_d);
			}
		}
	}
}
