use std::io::{self, Write};

use clap::{Arg, Command};

use crate::{
    application::{
        CreateTodo, CreateTodoRequest, DeleteTodo, DeleteTodoRequest, IdGeneratore,
        MarkAsCompletedRequest, MarkAsCompletedTodo, ModifyTodo, ModifyTodoRequest, ReadTodes,
        TodoRepository, TodoUsecase,
    },
    infrastructure::TodoRepoInFile,
};


pub struct TodoCli {
    id_gen: Box<dyn IdGeneratore>,
    repo: Box<dyn TodoRepository>,
}

impl TodoCli {
    pub fn new(id_gen: Box<dyn IdGeneratore>, repo: Box<dyn TodoRepository>) -> Self {
        Self { id_gen, repo }
    }
}

impl TodoCli {
    pub fn run(self) {
        let matches = Self::todo().arg_required_else_help(true).get_matches();

       

        if let Some(create_matches) = matches.subcommand_matches("create") {
            let mut create_usecase = CreateTodo::new(self.repo, self.id_gen);
            let title = create_matches.get_one::<String>("title");
            let description = create_matches.get_one::<String>("description").unwrap();
            match title {
                Some(title) => {
                    let req = CreateTodoRequest {
                        title: title.clone(),
                        description: description.clone(),
                    };
                    let id = create_usecase.execute(req);
                    println!("{id} : Successfully added to your todo list")
                }
                None => {
                    print!("Enter a title for your todo task: ");
                    let mut title = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut title)
                        .expect("Failed to read line");
                    print!("Enter description for your todo task: ");
                    let mut description = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut description)
                        .expect("Failed to read line");
                    let req = CreateTodoRequest {
                        title: title.trim().to_string(),
                        description: description.trim().to_string(),
                    };
                    let id = create_usecase.execute(req);
                    println!("{id} : Successfully added to your todo list");
                }
            }
        }

        
       else if let Some(modify_matches) = matches.subcommand_matches("modify") {
            let mut modify_usecase = ModifyTodo::new(self.repo);
            let id = modify_matches.get_one::<String>("id").unwrap();
            let modify_title = modify_matches.get_one::<String>("title");
            let modify_description = modify_matches.get_one::<String>("description").unwrap();
            match modify_title {
                Some(title) => {
                    let req = ModifyTodoRequest {
                        id: id.clone(),
                        title: title.clone(),
                        description: modify_description.clone(),
                    };
                    let message = modify_usecase.execute(req);
                    println!("{message}")
                }
                None => {
                    print!("Enter a title for your todo task: ");
                    let mut title = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut title)
                        .expect("Failed to read line");
                    print!("Enter description for your todo task: ");
                    let mut description = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut description)
                        .expect("Failed to read line");
                    let req = ModifyTodoRequest {
                        id: id.trim().to_string(),
                        title: title.trim().to_string(),
                        description: description.trim().to_string(),
                    };

                    let message = modify_usecase.execute(req);
                    println!("{message}");
                }
            }
        }
      
      
        else if let Some(delete_matches) = matches.subcommand_matches("delete") {
            let mut delete_usecase = DeleteTodo::new(self.repo);
            let all = delete_matches.get_flag("all");
            match delete_matches.get_one::<String>("id") {
                Some(id) => {
                    let req = DeleteTodoRequest { id: id.clone() };
                    let message = delete_usecase.execute(req);
                    println!("{message}")
                }
                None => {
                    if all {
                        let repo = Box::new(TodoRepoInFile::new("data.json"));
                        let message = repo.remove();
                        println!("{message}")
                    }
                }
            };
        }
       
       
        else if let Some(read_matches) = matches.subcommand_matches("read") {
            let mut read_usecase = ReadTodes::new(self.repo);
            let id = read_matches.get_one::<String>("id");

            let todos = read_usecase.execute(id.cloned());
            println!("{:#?}", todos)
        }
        let repo = Box::new(TodoRepoInFile::new("data.json"));
        let mut completed_usecase = MarkAsCompletedTodo::new(repo);

        if let Some(mark_as_completed) = matches.subcommand_matches("completed") {
            let id = mark_as_completed.get_one::<String>("id").unwrap();
            let req = MarkAsCompletedRequest {
                id: id.clone(),
                is_completed: true,
            };
            let message = completed_usecase.execute(req);
            println!("{message}")
        }
    }
}

trait TodoCommand {
    fn todo() -> Command;
    fn create() -> Command;
    fn modify() -> Command;
    fn delete() -> Command;
    fn read() -> Command;
    fn completed() -> Command;
}
impl TodoCommand for TodoCli {
    fn todo() -> Command {
        Command::new("todo")
            .version("1.0")
            .author("Sayed Ali sina Hussaini")
            .about("This an ab")
            .subcommand(Self::create())
            .subcommand(Self::read())
            .subcommand(Self::delete())
            .subcommand(Self::modify())
            .subcommand(Self::completed())
    }

    fn create() -> Command {
        let option_title = Arg::new("title")
            .help("add title to your todo task")
            .short('t')
            .long("title");
        let option_description = Arg::new("description")
            .help("add description to your todo task")
            .short('d')
            .long("description")
            .default_value("");
        Command::new("create")
            .aliases(["add", "insert", "push", "write"])
            .arg(option_title)
            .arg(option_description)
    }

    fn modify() -> Command {
        let option_title = Arg::new("title")
            .help("add title to your todo task")
            .short('t')
            .long("title");
        let option_description = Arg::new("description")
            .help("add description to your todo task")
            .short('d')
            .long("description")
            .default_value("");
        let argumant_id = Arg::new("id").required(true);
        Command::new("modify")
            .aliases(["update"])
            .arg(argumant_id)
            .arg(option_title)
            .arg(option_description)
            .arg_required_else_help(true)
    }

    fn delete() -> Command {
        let argumant_id = Arg::new("id").required_unless_present("all");
        let flag_all = Arg::new("all")
            .action(clap::ArgAction::SetTrue)
            .long("all")
            .help("flag ");
        Command::new("delete")
            .aliases(["remove", "rm"])
            .arg(argumant_id)
            .arg(flag_all)
            .arg_required_else_help(true)
    }

    fn read() -> Command {
        let argumant_id = Arg::new("id");
        Command::new("read")
            .aliases(["show", "ls", "preview"])
            .arg(argumant_id)
    }

    fn completed() -> Command {
        let argumant_id = Arg::new("id");
        Command::new("completed")
            .aliases(["complet", "finish"])
            .arg(argumant_id)
            .arg_required_else_help(true)
    }
}
