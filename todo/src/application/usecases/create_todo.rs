

use crate::{application::{TodoRepository, IdGeneratore, CreateTodoRequest}, domain::Todo};

use super::TodoUsecase;

pub struct CreateTodo {
    repo: Box<dyn TodoRepository>,
    id_gen: Box<dyn IdGeneratore>,
}

impl CreateTodo {
    pub fn new(repo: Box<dyn TodoRepository>, id_gen: Box<dyn IdGeneratore>) -> Self {
        Self { repo, id_gen }
    }
}

impl TodoUsecase<CreateTodoRequest, String> for CreateTodo {
    fn execute(&mut self, request: CreateTodoRequest) -> String {
        let id = self.id_gen.generate();
        let new_todo = Todo::new()
            .set_id(id.clone())
            .set_title(request.title)
            .set_description(request.description)
            .set_is_completed(false);
        
        self.repo.create_todo( new_todo);
        id
    }
}
