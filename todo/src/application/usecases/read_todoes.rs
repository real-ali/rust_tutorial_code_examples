use serde_json::de::Read;

use crate::{application::TodoRepository, domain::Todo};

use super::TodoUsecase;

pub struct  ReadTodes{
    repo: Box<dyn TodoRepository>
}

impl ReadTodes {
    pub fn new(repo: Box<dyn TodoRepository>) -> Self { Self { repo } }
}

impl TodoUsecase<Option<String>,Vec<Todo>> for ReadTodes {
    fn execute(&mut self, request: Option<String>) -> Vec<Todo> {
        match request {
            Some(id)=>{
              self.repo.read_todoes().iter()
              .filter(|todo| todo.id == Some(id.to_string()))
              .cloned()
              .collect()
            },
            None=>{
                self.repo.read_todoes()
            }
        }
        
        
    }
}

