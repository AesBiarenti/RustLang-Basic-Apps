fn main() {
    
    let todo1 = Todo {
        content: "Mailleri Kontrol Et".to_string(),
        is_finished: false,
    };
    let todo2 = Todo {
        content: "Task'leri tamamla".to_string(),
        is_finished: false,
    };
    let todo3 = Todo {
        content: "Yememk Yap".to_string(),
        is_finished: false,
    };
    let todo4 = Todo {
        content: "Spor Yap".to_string(),
        is_finished: false,
    };
    let mut todo_list = TodoList::new();
    todo_list.add_todo(todo1);
    todo_list.add_todo(todo2);
    todo_list.add_todo(todo3);
    todo_list.add_todo(todo4);
    println!("\n--- Başlangıç Todo Listesi ---");
    todo_list.list_todo();
    todo_list.update_todo(0, None, Some(true));
    todo_list.update_todo(1, Some("Task ve Case leri tamamla".to_string()), None);
    println!("\n--- Güncellenmiş Todo Listesi ---");
    todo_list.list_todo();
    println!("\n--- 3. İndexi Silinmiş Todo Listesi ---");
    todo_list.delete_todo(4);

    todo_list.list_todo();
}
struct Todo {
    content: String,
    is_finished: bool,
}
struct TodoList {
    todos: Vec<Todo>,
}
impl TodoList {
    fn new() -> Self {
        TodoList {
            todos: (Vec::new()),
        }
    }
    fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }
    fn update_todo(&mut self, index: usize, new_content: Option<String>, new_status: Option<bool>) {
        if let Some(todo) = self.todos.get_mut(index) {
            if let Some(content) = new_content {
                todo.content = content;
            }
            if let Some(status) = new_status {
                todo.is_finished = status
            }
            println!("Todo Güncellendi")
        } else {
            println!("Güncelleme Başarısız")
        }
    }
    fn delete_todo(&mut self, index: usize) {
        if index < self.todos.len() {
            self.todos.remove(index);
            println!("Todo Silindi");
        } else {
            println!("Geçersiz İndex: Silme işlemi başarısız");
        }
    }
    fn list_todo(&self) {
        for (index, todo) in self.todos.iter().enumerate() {
            println!(
                "{}. {} {}",
                index,
                todo.content,
                if todo.is_finished {
                    "Tamamlandı"
                } else {
                    "Tamamlanmadı"
                }
            )
        }
    }
}
