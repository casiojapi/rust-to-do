use std::{collections::HashMap, fs::OpenOptions};
use std::io::Read;
use std::str::FromStr;

struct TodoList {
    map: HashMap<String, bool>,
}

impl TodoList {
    fn new() -> Result<TodoList, std::io::Error> {
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t',).collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k,v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        Ok(TodoList {map})
    }
    fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = true),
            None => None,
        }
    }
}
fn main() {

    

    //while (action == "add" || action == "done" || action == "view")
    let action = std::env::args().nth(1).expect("que accion?");
    let item = std::env::args().nth(2).expect("que item?");

    println!("action: {:?}, item: {:?}.", action, item);

    let mut todo_list = TodoList::new().expect("inicializo mal la \"db\"");

    if action == "add" {
        todo_list.insert(item);
        match todo_list.save() {
            Ok(_) => println!("to-do list guardada en \"db.txt\"."),
            Err(msg) => println!("error: {}", msg),
        };
    } else if action == "done" {
        match todo_list.complete(&item) {
            None => println!("'{}' no existe.", item),
            Some(_) => match todo_list.save() {
                Ok(_) => println!("guardado"),
                Err(msg) => println!("error guardando el done: {}", msg),
            },

        }
    }


}
