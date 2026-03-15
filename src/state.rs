use std::sync::Arc;

use crate::database::Todo;
use dashmap::mapref::one::RefMut;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::establish_connection,
    error::{Error, Result},
};

//crud

pub struct AppState {
    db: PgConnection,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        AppState {
            db: establish_connection(),
        }
        .into()
    }

    pub fn create_todo(&mut self, msg: String) -> Result<Uuid> {
        use crate::schema::todos::dsl::*;
        let new_todo = Todo::new(msg);
        let new_uuid = new_todo.id;

        diesel::insert_into(todos)
            .values(new_todo)
            .execute(&mut self.db)?;

        Ok(new_uuid)
    }

    pub fn list_todos(&mut self) -> Result<Vec<Todo>> {
        use crate::schema::todos::dsl::*;

        let all_todos = todos.load(&mut self.db)?;
        Ok(all_todos)
    }

    pub fn update_todo(&mut self, target_id: Uuid, new_message: String) -> Result<()> {
        use crate::schema::todos::dsl::*;

        diesel::update(todos.filter(id.eq(target_id)))
            .set(message.eq(new_message))
            .execute(&mut self.db)?;

        Ok(())
    }

    pub fn find_todo_with_id(&mut self, target_id: Uuid) -> Result<Option<Todo>> {
        use crate::schema::todos::dsl::*;

        match todos.find(target_id).first(&mut self.db) {
            Ok(target_todo) => Ok(Some(target_todo)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            _ => Err(Error::DatabaseError),
        }
    }

    pub fn delete_todo(&mut self, target_id: Uuid) -> Result<()> {
        use crate::schema::todos::dsl::*;

        diesel::delete(todos.filter(id.eq(target_id))).execute(&mut self.db)?;

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
        state
            .update_todo(wrong_id, updated_message.clone())
            .unwrap_err();

        // delete_todo
        state.delete_todo(id).unwrap();
        assert!(state.find_todo_with_id(id).is_err());
        assert_eq!(state.list_todos().count(), 0);

        // use wrong id to delete
        state.delete_todo(wrong_id).unwrap_err();
    }
}
