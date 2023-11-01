use todo::{
    application::{CreateTodo, CreateTodoRequest, TodoUsecase, MarkAsCompletedTodo, MarkAsCompletedRequest, ReadTodes},
    infrastructure::{IDGeneratorImpl, TodoRepoInFile}, domain::Todo, presentation::TodoCli,
};

fn main() {
    let repo = Box::new(TodoRepoInFile::new("data.json"));
    let id_gen = Box::new(IDGeneratorImpl::new());
    TodoCli::new(repo, id_gen).run();
    

    // // let mut create_usecase = CreateTodo::new(repo, id_gen);
    // let mut ca = ReadTodes::new(repo);

    // let req = MarkAsCompletedRequest {
    //     id:"TODO76476016".into(),
    //    is_completed:true
    // };
    // let a =ca.execute(None);
    // print!("{a:#?}")
}
