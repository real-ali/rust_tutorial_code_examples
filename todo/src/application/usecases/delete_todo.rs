use crate::application::{TodoRepository, DeleteTodoRequest};

use super::TodoUsecase;

pub struct DeleteTodo {
    repo: Box<dyn TodoRepository>,
  
}

impl DeleteTodo {
    pub fn new(repo: Box<dyn TodoRepository>) -> Self {
        Self { repo}
    }
}

impl TodoUsecase<DeleteTodoRequest,String> for DeleteTodo {
    fn execute(&mut self, request: DeleteTodoRequest) -> String {
        self.repo.delete_todo(&request.id);
        String::from("Successfully deleted")
    }
}