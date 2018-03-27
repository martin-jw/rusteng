macro_rules! entity_manager {
    ($size:expr, $($name:ident : $comp:path),*) => {{
    	
    	pub struct EntityManager {
    		next_id: u32,
    		$(
    			pub $name: Storage<$comp>
    		)*
    	}

    	impl EntityManager {

    		pub fn get_next_entity(&mut self) -> u32 {
    			self.next_id
    		}

    		$(
    		pub fn add_component(&mut self, entity: u32, component: $comp) {
    			self.$name.add(entity, component)
    		}
    		)*
    	}

    	EntityManager {
    		next_id: 0,
    		$(
    			$name: Storage::<$comp>::new($size)
    		)*
    	}
    }}
}