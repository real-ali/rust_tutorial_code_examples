use crate::application::{MarkAsCompletedRequest, TodoRepository};

use super::TodoUsecase;

pub struct MarkAsCompletedTodo {
    repo: Box<dyn TodoRepository>,
}

impl MarkAsCompletedTodo {
    pub fn new(repo: Box<dyn TodoRepository>) -> Self {
        Self { repo }
    }
}

impl TodoUsecase<MarkAsCompletedRequest, String> for MarkAsCompletedTodo {
    fn execute(&mut self, request: MarkAsCompletedRequest) -> String {
        self.repo.mark_as_completed(&request.id, request.is_completed);
        String::from("Task marked as completed")
    }
}
