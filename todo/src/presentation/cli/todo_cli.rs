use clap::Command;

use crate::application::{IdGeneratore, TodoRepository};

pub struct TodoCli {
    repo: Box<dyn TodoRepository>,
    id_gen: Box<dyn IdGeneratore>,
}

impl TodoCli {
    pub fn new(repo: Box<dyn TodoRepository>, id_gen: Box<dyn IdGeneratore>) -> Self {
        Self { repo, id_gen }
    }
    pub fn run(self) {
        Self::todo().arg_required_else_help(true).get_matches();
    }
}

trait TodoCommand {
    fn todo() -> Command;
    fn create() -> Command;
    fn modify() -> Command;
    fn delete() -> Command;
    fn read() -> Command;
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
    }

    fn create() -> Command {
        Command::new("create")
            .about("about")
            .aliases(["add", "insert", "push"])
    }

    fn modify() -> Command {
        Command::new("modify").about("about").aliases(["update"])
    }

    fn delete() -> Command {
        Command::new("delete")
            .about("about")
            .aliases(["remove", "rm"])
    }

    fn read() -> Command {
        Command::new("read")
            .about("about")
            .aliases(["show", "ls", "preview"])
    }
}
