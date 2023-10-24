use serde::{Deserialize, Serialize};
use serde_json;
use std::time::SystemTime;
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
// Creamos una estructura con los datos del Todo
struct Todo {
    id: u8,
    nombre: String,
    completado: bool,
    fecha_creacion: SystemTime,
}

#[derive(Debug, Serialize, Deserialize)]
// Creamos una lista con un vector de Todos
struct ListaTodo {
    todos: Vec<Todo>,
}

// Generamos las funciones asociadas a la estructura de Todo
impl Todo {
    // Constructor
    fn new(id: u8, nombre: String) -> Todo {
        Todo {
            id,
            nombre,
            completado: false,
            fecha_creacion: SystemTime::now(),
        }
    }
    // Completar Todo
    fn completar(&mut self) {
        self.completado = true;
    }
}

// Generamos las funciones asociadas a la estructura de ListaTodo
impl ListaTodo {
    // Constructor
    fn new() -> ListaTodo {
        ListaTodo { todos: Vec::new() }
    }
    // Añadir Todo a la lista
    fn anadir(&mut self, id: u8, nombre: String) {
        let todo = Todo::new(id, nombre);
        self.todos.push(todo);
    }
    // Completar Todo de la lista
    fn completar(&mut self, id: u8) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completar();
        }
    }
    // Buscar Todo por id en la lista
    fn buscar_por_id(&self, id: u8) -> Option<&Todo> {
        self.todos.iter().find(|t| t.id == id)
    }
}

fn main() {
    let mut todo_list = ListaTodo::new();

    todo_list.anadir(1, "Comprar pan".to_string());
    todo_list.anadir(2, "Comprar café".to_string());

    println!("Lista:");
    for todo in &todo_list.todos {
        let json = serde_json::to_string(&todo).unwrap();
        println!("{}", json);
    }

    todo_list.completar(1);

    if let Some(todo) = todo_list.buscar_por_id(1) {
        println!("Completeda: {:?}", todo);
    }
    let json = serde_json::to_string(&todo_list).unwrap();
    println!("JSON: {}", json);
}
