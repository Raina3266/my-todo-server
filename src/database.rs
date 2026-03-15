use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[derive(Queryable, Selectable, Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: Uuid,
    pub message: String,
}

impl Todo {
    pub fn new(message: String) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            message,
        }
    }
}