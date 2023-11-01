use crate::{
    application::{ModifyTodoRequest, TodoRepository},
    domain::Todo,
};

use super::TodoUsecase;

pub struct ModifyTodo {
    repo: Box<dyn TodoRepository>,
}

impl ModifyTodo {
    pub fn new(repo: Box<dyn TodoRepository>) -> Self {
        Self { repo }
    }
}

impl TodoUsecase<ModifyTodoRequest, String> for ModifyTodo {
    fn execute(&mut self, request: ModifyTodoRequest) -> String {
        let new_data = Todo::new()
            .set_title(request.title)
            .set_description(request.description);
        self.repo.modify_todo(&request.id, new_data);
        String::from("Modified Successfully")
    }
}
