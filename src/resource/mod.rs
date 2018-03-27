use std::collections::HashMap;
use std::any::Any;

mod text;

pub trait Loader {
	fn load(&self, path: &str) -> Result<Box<Any>, &str>;
}

pub struct ResourceLocation<'a> {
	pub path: &'a str,
	pub loader: &'a str,
	pub name: &'a str
}

pub struct AssetManager<'a> {
	loaders: HashMap<&'a str, &'a Loader>,
	assets: HashMap<&'a str, Box<Any>>,
	locations: Vec<ResourceLocation<'a>>
}

impl<'a> AssetManager<'a> {

	pub fn new() -> AssetManager<'a> {
		let mut manager = AssetManager {
			loaders: HashMap::new(),
			assets: HashMap::new(),
			locations: Vec::new()
		};

		manager.add_loader("text", &text::TextLoader);
		manager
	}

	pub fn add_loader(&mut self, name: &'a str, loader: &'a Loader) {
		self.loaders.insert(name, loader);
	}

	pub fn add_location(&mut self, location: ResourceLocation<'a>) {
		self.locations.push(location);
	}

	pub fn get(&self, asset: &'a str) -> Option<&Box<Any>> {
		self.assets.get(asset)
	}

	pub fn load_all(&mut self) {

		for location in &self.locations {
			if let Some(loader) = self.loaders.get(location.loader) {
				match loader.load(location.path) {
					Ok(b) => {
						self.assets.insert(location.name, b);
					}
					Err(error) => {
						println!("Unable to load resource {}. Loader returned with error: {}", location.name, error);
					}
				}
			}
			else {
				println!("Loader {} does not exist. Ignoring resource location: {}", location.loader, location.name)
			}
		}
	}
}