use self::errors::TodoError;

pub mod errors;

pub enum TodoStatus {
    PENDING,
    INPROGRESS,
    DONE,
}

pub struct Todo {
    id: u64,
    name: String,
    desc: String,
    status: TodoStatus,
    start: String,
    end: String,
    version: u64,
    created_at: String,
    updated_at: String,
}

pub trait Storage {
    fn get(&self, id: u64) -> Result<Todo, TodoError>;
    fn create(&self, todo: Todo) -> Result<Todo, TodoError>;
    fn update(&self, id: u64, version: u64, new_todo: Todo) -> Result<Todo, TodoError>;
    fn delete(&self, id: u64) -> Result<Todo, TodoError>;
}
