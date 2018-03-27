use resource::Loader;
use std::any::Any;
use std::fs::File;
use std::io::Read;

pub struct TextLoader;

impl Loader for TextLoader {

	fn load(&self, path: &str) -> Result<Box<Any>, &str> {

		if let Some(mut file) = File::open(path).ok() {
			let mut string = String::new();
			match file.read_to_string(&mut string) {
				Ok(_) => {
					Ok(Box::new(string))
				}
				_ => {
					Err("Unable to read file!")
				}
			}
		}
		else {
			Err("Unable to open file!")
		}
	}
}