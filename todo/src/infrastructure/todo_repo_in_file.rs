use std::{
    fs::{self, File, OpenOptions},
    io::{ Read, Write},
};

use crate::{
    application::TodoRepository,
    domain::Todo,
};

pub struct TodoRepoInFile {
    path: String,
}

impl TodoRepoInFile {
    pub fn new(path: &str) -> Self {
        if let Err(_) = fs::File::open(&path.to_string()) {
            fs::File::create(String::from(path)).expect("Creation Faild!");
        }
        Self {
            path: path.to_string(),
        }
    }
}

impl TodoRepository for TodoRepoInFile {
    fn read_todoes(&mut self) -> Vec<Todo> {
        let mut file = File::open(&self.path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let todos: Vec<Todo> = serde_json::from_str(&content).unwrap();
        todos
    }
    fn create_todo(&mut self, todo: Todo) {
        let todoes: &mut Vec<Todo> = &mut Vec::<Todo>::new();
        todoes.append(&mut self.read_todoes());
        todoes.append(&mut vec![todo]);

        let todos_json = serde_json::to_string_pretty(todoes).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .unwrap();
        file.write_all(todos_json.as_bytes()).unwrap();
    }

    fn delete_todo(&mut self, id: &str) {
        let todoes: &mut Vec<Todo> = &mut Vec::<Todo>::new();
        todoes.append(&mut self.read_todoes());
        todoes.retain(|t| t.id == Some(id.to_string()));
        let todos_json = serde_json::to_string_pretty(todoes).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .unwrap();
        file.write_all(todos_json.as_bytes()).unwrap();
    }
    fn modify_todo(&mut self, id: &str, new_data: Todo) {
        let todoes: &mut Vec<Todo> = &mut Vec::<Todo>::new();
        todoes.append(&mut self.read_todoes());
        if let Some(data) = todoes
            .iter_mut()
            .find(|todo| todo.id == Some(id.to_string()))
        {
            data.title = new_data.title;
            data.description = new_data.description;
        }
        let todos_json = serde_json::to_string_pretty(todoes).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .unwrap();
        file.write_all(todos_json.as_bytes()).unwrap();
    }

    fn mark_as_completed(&mut self, id: &str, is_completed: bool) {
        let todoes: &mut Vec<Todo> = &mut Vec::<Todo>::new();
        todoes.append(&mut self.read_todoes());
        if let Some(data) = todoes
            .iter_mut()
            .find(|todo| todo.id == Some(id.to_string()))
        {
            data.is_completed = is_completed
           
        }
        let todos_json = serde_json::to_string_pretty(todoes).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .unwrap();
        file.write_all(todos_json.as_bytes()).unwrap();
    }
}
