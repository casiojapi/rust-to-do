use std::collections::HashMap;



struct TodoList {
    map: HashMap<String, bool>,
}

impl TodoList {
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
}
fn main() {

    


    let action = std::env::args().nth(1).expect("que accion?");
    let item = std::env::args().nth(2).expect("que item?");

    println!("action: {:?}, item: {:?}.", action, item);

    let mut todo_list = TodoList {
        map: HashMap::new(),
    };
    
    if action == "add" {
        todo_list.insert(item);
        match todo_list.save() {
            Ok(_) => println!("to-do list guardada en \"db.txt\"."),
            Err(msg) => println!("error: {}", msg),
        };
    }


}
