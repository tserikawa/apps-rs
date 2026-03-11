use crate::filter::Filter;

#[derive(Clone)]
pub struct ToDo {
    pub id: usize,
    pub content: String,
    pub completed: bool,
}

impl ToDo {
    pub fn new(id: usize, content: String, completed: bool) -> Self {
        ToDo { id, content, completed }
    }

    pub fn is_show(&self, filter: &Filter) -> bool {
        match filter {
            Filter::All => true,
            Filter::Completed => self.completed,
            Filter::NotCompleted => !self.completed
        }
    }
}

impl Default for ToDo {
    fn default() -> Self {
        Self {
            id: 0,
            content: String::new(),
            completed: false,
        }
    }
}

pub struct ToDoCollection {
    id: usize,
    todos: Vec<ToDo>
}

impl ToDoCollection {
    pub fn new() -> Self {
        ToDoCollection {id: 1, todos: Vec::new() }
    }

    pub fn add(&mut self, content: String, completed: bool) {
        self.todos.push(ToDo::new(self.id, content, completed));
        self.id += 1;
    }

    pub fn delete(&mut self, ids: &Vec<usize>) {
        self.todos = self.todos.iter().cloned().filter(|f| !ids.contains(&f.id)).collect();
    }

    pub fn items_mut(&mut self, filter: &Filter) -> Vec<&mut ToDo> {
        // 可変にするのはVec<T>そのものではなく、Vec<T>の各要素
        self.todos.iter_mut().filter(|f| f.is_show(filter)).collect()
    }
}

impl Default for ToDoCollection {
    fn default() -> Self {
        ToDoCollection::new()
    }
}