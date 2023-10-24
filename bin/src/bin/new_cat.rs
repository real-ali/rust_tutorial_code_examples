use std::{
    fmt::Write,
    io::{BufRead, BufReader, Read},
};

fn main() {
    let mut commands: Vec<String> = std::env::args().collect();
    commands.remove(0);
    let result = CatCommand::new(Some(commands)).format();
    let mut buffer = String::new();
    let mut line_counter = 0;
    match result {
        CatCommandState::WithFlagN(args) => {
            
            for  arg in args.iter() {
               
                let  cont = std::fs::File::open(arg).unwrap();
                let reader = BufReader::new(cont);
                for line in reader.lines(){
                    let line = line.unwrap();
                    line_counter+=1;
                    buffer.write_str(format!("{line_counter} {line}\n").as_str()).unwrap();
              
                }
            }
            println!("{buffer}")
        }
        CatCommandState::WithFlagP(args) => {
            for  arg in args{
                let  cont = std::fs::File::open(arg).unwrap();
                let reader = BufReader::new(cont);
               
                for line in reader.lines() {
                    let line = line.unwrap();
                    if !line.trim().is_empty() {
                        line_counter += 1;
                        buffer.write_str(format!("{line_counter} {line}\n").as_str()).unwrap();
                    } else {
                        buffer.write_str(format!("{line}\n").as_str()).unwrap();
                    }
                }
            }
            println!("{buffer}")
        }
        CatCommandState::WithoutFlag(args) => {
            for arg in args {
                let mut cont = std::fs::File::open(arg).unwrap();
                cont.read_to_string(&mut buffer).unwrap();
                buffer.write_str("\n").unwrap();
            }
        }
        CatCommandState::Empty => loop {
            std::io::stdin().read_line(&mut buffer).unwrap();
        },
    }
   
}
struct CatCommand {
    command: Option<Vec<String>>,
}
enum CatCommandState {
    WithFlagN(Vec<String>),
    WithFlagP(Vec<String>),
    WithoutFlag(Vec<String>),
    Empty,
}

impl CatCommand {
    fn new(commons: Option<Vec<String>>) -> Self {
        Self { command: commons }
    }
    fn format(self) -> CatCommandState {
        match self.command {
            Some(commands) => {
                if commands.len() > 1 && commands[0].contains("-n") {
                    return CatCommandState::WithFlagN(commands[1..commands.len()].to_vec());
                } else if commands.len() > 1 && commands[0].contains("-p") {
                    return CatCommandState::WithFlagP(commands[1..commands.len()].to_vec());
                } else if commands.len() > 0 {
                    return CatCommandState::WithoutFlag(commands);
                } else {
                    return CatCommandState::Empty;
                }
            }
            None => CatCommandState::Empty,
        }
    }
}
