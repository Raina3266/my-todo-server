use std::{collections::HashMap, sync::Arc};

use dashmap::{DashMap, mapref::one::RefMut};
use serde::Serialize;
use uuid::Uuid;

use crate::error::{Error, Result};

//crud
#[derive(Clone, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub message: String,
}

impl Todo {
    fn new(message: String) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            message,
        }
    }
}

pub struct AppState {
    todos: DashMap<Uuid, Todo>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        AppState {
            todos: DashMap::new(),
        }
        .into()
    }

    pub fn create_todo(&self, message: String) -> Uuid {
        let new_todo = Todo::new(message);
        let new_uuid = new_todo.id;
        self.todos.insert(new_uuid, new_todo);
        new_uuid
    }

    pub fn list_todos(&self) -> impl Iterator<Item = Todo> {
        self.todos.iter().map(|r| r.value().clone())
    }

    pub fn update_todo(&self, id: Uuid, new_message: String) -> Result<()> {
        let mut old_todo = self.find_todo_with_id(id)?;
        old_todo.message = new_message;

        Ok(())
    }

    pub fn find_todo_with_id(&self, id: Uuid) -> Result<RefMut<'_, Uuid, Todo>> {
        self.todos.get_mut(&id).ok_or(Error::TodoNotFound)
    }

    pub fn delete_todo(&self, id: Uuid) -> Result<()> {
        self.todos.remove(&id).ok_or(Error::TodoNotFound)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_crud() {
        let message = String::from("hello");
        let wrong_id = Uuid::new_v4();
        let state = AppState::new();

        // create_todo
        let id = state.create_todo(message.clone());
        assert!(state.find_todo_with_id(id).is_ok());
        assert!(state.find_todo_with_id(wrong_id).is_err());
        assert_eq!(state.list_todos().count(), 1);
        assert_eq!(state.list_todos().next().unwrap().message, message);

        // update_todo
        let updated_message = String::from("world");

        assert!(state.find_todo_with_id(id).is_ok());
        state.update_todo(id, updated_message.clone()).unwrap();
        assert_eq!(state.list_todos().count(), 1);
        assert_eq!(state.list_todos().next().unwrap().message, updated_message);
        
        // use wrong id to update
        state.update_todo(wrong_id, updated_message.clone()).unwrap_err();

        // delete_todo
        state.delete_todo(id).unwrap();
        assert!(state.find_todo_with_id(id).is_err());
        assert_eq!(state.list_todos().count(), 0);
        
        // use wrong id to delete
        state.delete_todo(wrong_id).unwrap_err();
    }
}
