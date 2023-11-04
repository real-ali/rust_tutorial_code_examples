use todo::{infrastructure::IDGeneratorImpl, presentation::TodoCli};



fn main() {
    // let repo = Box::new(TodoRepoInFile::new("data.json")).as_ref();
    let id_gen = Box::new(IDGeneratorImpl::new());
    TodoCli::new(id_gen).run();
}
