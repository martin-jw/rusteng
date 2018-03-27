extern crate nalgebra as na;
use self::na::{ Vector2, Rotation2 };

use std::rc::{ Rc, Weak };
use std::collections::{ HashMap };

pub trait Component {
	fn create() -> Self;
}

pub struct TransformComponent {
	pub position: Vector2<f32>,
	pub rotation: Rotation2<f32>
}
impl Component for TransformComponent {

	fn create() -> Self {
		TransformComponent{
			position: Vector2::identity(),
			rotation: Rotation2::identity()
		}
	}
}

struct StorageEntry<T: ?Sized> {
	entry: Option<Rc<T>>,
	prev_index: isize
}

pub struct Storage<T: ?Sized> {
	store: Vec<StorageEntry<T>>,
	mapping: HashMap<u32, usize>,
	free_index: usize
}

impl<T> Storage<T> {

	pub fn new(size: usize) -> Storage<T> {

		let mut store = Vec::with_capacity(size);
		for _i in 0..size {
			store.push(StorageEntry::<T>{
				entry: None,
				prev_index: -1
			});
		}

		Storage {
			store: store,
			mapping: HashMap::with_capacity(size),
			free_index: 0
		}
	}

	pub fn add(&mut self, id: u32, entry: T) {

		println!("Adding to index: {}", self.free_index);

		let next_index = self.store[self.free_index].prev_index;
		self.store[self.free_index].entry = Some(Rc::new(entry));
		self.mapping.insert(id, self.free_index);

		if next_index >= 0 {
			self.free_index = next_index as usize;
		}
		else {
			self.free_index += 1;
		}
	}

	pub fn remove(&mut self, id: u32) {

		let index = *self.mapping.get(&id).unwrap();
		self.store[index].entry = None;
		self.store[index].prev_index = self.free_index as isize;

		self.free_index = index;

		self.mapping.remove(&id);
	}

	pub fn get(&self, id: u32) -> Option<Rc<T>> {

		match self.mapping.get(&id) {
			Some(index) => {
				match self.store.get(*index).unwrap().entry {
					Some(ref cell) => {
						Some(Rc::clone(cell))
					}
					None => {
						None
					}
				}
			}
			None => {
				None
			}
		}
	}

	pub fn get_weak(&self, id: u32) -> Option<Weak<T>> {
		if let Some(rc) = self.get(id) {
			Some(Rc::downgrade(&rc))
		}
		else {
			None
		}
	}
}