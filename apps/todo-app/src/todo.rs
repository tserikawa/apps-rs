use crate::filter::Filter;

#[derive(Clone)]
pub struct ToDoItem {
    pub id: usize,
    pub content: String,
    pub completed: bool,
}

impl ToDoItem {
    pub fn new(id: usize, content: String, completed: bool) -> Self {
        ToDoItem { id, content, completed }
    }

    pub fn is_show(&self, filter: &Filter) -> bool {
        match filter {
            Filter::All => true,
            Filter::Completed => self.completed,
            Filter::NotCompleted => !self.completed
        }
    }
}

impl Default for ToDoItem {
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
    items: Vec<ToDoItem>
}

impl ToDoCollection {
    pub fn new() -> Self {
        ToDoCollection {id: 1, items: Vec::new() }
    }

    pub fn add(&mut self, content: String, completed: bool) {
        self.items.push(ToDoItem::new(self.id, content, completed));
        self.id += 1;
    }

    pub fn delete(&mut self, ids: &Vec<usize>) {
        self.items = self.items.iter().cloned().filter(|f| !ids.contains(&f.id)).collect();
    }

    pub fn items_mut(&mut self, filter: &Filter) -> Vec<&mut ToDoItem> {
        // 可変にするのはVec<T>そのものではなく、Vec<T>の各要素
        self.items.iter_mut().filter(|f| f.is_show(filter)).collect()
    }
}

impl Default for ToDoCollection {
    fn default() -> Self {
        ToDoCollection::new()
    }
}