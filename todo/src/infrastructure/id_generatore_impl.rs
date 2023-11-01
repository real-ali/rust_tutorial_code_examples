use rand::Rng;

use crate::application::IdGeneratore;





pub struct IDGeneratorImpl;

impl IDGeneratorImpl {
    pub fn new() -> Self {
        Self
    }
}

impl IdGeneratore for IDGeneratorImpl {
    fn generate(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str("TODO");
        let id = rand::thread_rng().gen_range(100000..=100000000);
        buffer.push_str(&format!("{id}"));
        buffer
    }
}