use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
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

    use axum_test::TestServer;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn test_create_todo() {
        let state = AppState::new();
        let app = make_router(state);
        let server = TestServer::new(app);

        let response = server
            .post("/create_todo")
            .json(&json!({"message": "happy"}))
            .await;
        response.assert_status_ok();
        let body: HashMap<String, Uuid> = response.json();
        assert_eq!(body.len(), 1);
        body.get("id").unwrap();
    }

    #[tokio::test]
    async fn test_list_todo() {
        let state = AppState::new();
        let app = make_router(state);
        let server = TestServer::new(app);
        let _add_first_todo = server
            .post("/create_todo")
            .json(&json!({"message": "happy"}))
            .await;
        let _add_second_todo = server
            .post("/create_todo")
            .json(&json!({"message": "friday"}))
            .await;
        let response = server.get("/list_todos").await;
        response.assert_status_ok();
        let body: HashMap<String, Vec<Todo>> = response.json();
        let todos = body.get("todos").unwrap();
        assert_eq!(todos.len(), 2);
        assert_eq!(todos.iter().next().unwrap().message, "happy");
        assert_eq!(todos.iter().next().iter().next().unwrap().message, "friday");
    }

    #[tokio::test]
    async fn test_update_todo() {
        let state = AppState::new();
        let app = make_router(state);
        let server = TestServer::new(app);
        let add_first_todo = server
            .post("/create_todo")
            .json(&json!({"message": "happy"}))
            .await;
        let first_todo: HashMap<String, Uuid> = add_first_todo.json();
        let uuid = *first_todo.get("id").unwrap();
        let response = server
            .post("/update_todo")
            .json(&json!({"id": uuid, "message": "sad"}))
            .await;
        response.assert_status_ok();
        
        let response = server.get("/list_todos").await;
        let body: HashMap<String, Vec<Todo>> = response.json();
        let todos = body.get("todos").unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos.iter().next().unwrap().message, "sad");
    }
    
    #[tokio::test]
    async fn test_delete_todo() {
        let state = AppState::new();
        let app = make_router(state);
        let server = TestServer::new(app);
        let add_first_todo = server
            .post("/create_todo")
            .json(&json!({"message": "happy"}))
            .await;
        let first_todo: HashMap<String, Uuid> = add_first_todo.json();
        let uuid = *first_todo.get("id").unwrap();
        let response = server
            .post("/delete_todo")
            .json(&json!({"id": uuid}))
            .await;
        response.assert_status_ok();
        
        let response = server.get("/list_todos").await;
        let body: HashMap<String, Vec<Todo>> = response.json();
        let todos = body.get("todos").unwrap();
        assert_eq!(todos.len(), 0);
    }
    
}
