use crate::domain::Todo;





pub trait TodoRepository {
    fn read_todoes(&mut self)->Vec<Todo>;
    fn create_todo(&mut self, todo: Todo);
    fn delete_todo(&mut self, id: &str);
    fn modify_todo(&mut self,id: &str,new_data: Todo);
    fn mark_as_completed(&mut self, id: &str, is_completed: bool);
   
}
