use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use axum_test::TestServer;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    error::Result,
    state::{AppState, Todo},
};

mod error;
mod state;

fn make_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/create_todo", post(create_todo_handler))
        .route("/list_todos", get(list_todo_handler))
        .route("/update_todo", post(update_todo_handler))
        .route("/delete_todo", post(delete_todo_handler))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = make_router(state);
    


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


#[derive(Deserialize)]
struct CreateTodoRequest {
    message: String,
}

#[derive(Serialize)]
struct CreateTodoResponse {
    id: Uuid,
}

#[axum::debug_handler]
async fn create_todo_handler(
    State(state): State<Arc<AppState>>,
    Json(message): Json<CreateTodoRequest>,
) -> Json<CreateTodoResponse> {
    Json(CreateTodoResponse {
        id: state.create_todo(message.message),
    })
}

#[derive(Serialize)]
struct ListTodoResponse {
    todos: Vec<Todo>,
}

async fn list_todo_handler(State(state): State<Arc<AppState>>) -> Json<ListTodoResponse> {
    Json(ListTodoResponse {
        todos: state.list_todos().collect(),
    })
}

#[derive(Deserialize)]
struct UpdateTodoRequest {
    id: Uuid,
    message: String,
}

#[axum::debug_handler]
async fn update_todo_handler(
    State(state): State<Arc<AppState>>,
    Json(todo): Json<UpdateTodoRequest>,
) -> Result<()> {
    state.update_todo(todo.id, todo.message)?;
    Ok(())
}

#[derive(Deserialize)]
struct DeleteTodoRequest {
    id: Uuid,
}

#[axum::debug_handler]
async fn delete_todo_handler(
    State(state): State<Arc<AppState>>,
    Json(id): Json<DeleteTodoRequest>,
) -> Result<()> {
    state.delete_todo(id.id)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use axum::http::StatusCode;
    use serde_json::Value;

    use super::*;

    #[tokio::test]
    async fn test_name() {
        let state = AppState::new();
        let app = make_router(state);
        let server = TestServer::new(app);
        
        let response = server.post("/create_todo")
            .json(&json!({"message": "happy"}))
            .await;
        response.assert_status_ok();
        let body: HashMap<String, Uuid> = response.json();
        assert_eq!(body.len(), 1);
        body.get("id").unwrap();
    }
}