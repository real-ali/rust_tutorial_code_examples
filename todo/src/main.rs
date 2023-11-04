use todo::{infrastructure::{IDGeneratorImpl, TodoRepoInFile}, presentation::TodoCli};



fn main() {
    let repo = Box::new(TodoRepoInFile::new("data.json"));
    let id_gen = Box::new(IDGeneratorImpl::new());
    TodoCli::new(id_gen,repo).run();
}
