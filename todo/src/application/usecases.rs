mod create_todo;
mod delete_todo;
mod mark_as_completed_todo;
mod modify_todo;
mod read_todoes;

pub use create_todo::CreateTodo;
pub use delete_todo::DeleteTodo;
pub use mark_as_completed_todo::MarkAsCompletedTodo;
pub use modify_todo::ModifyTodo;
pub use read_todoes::ReadTodes;

pub trait TodoUsecase<Request, Response> {
    fn execute(&mut self, request: Request) -> Response;
}
