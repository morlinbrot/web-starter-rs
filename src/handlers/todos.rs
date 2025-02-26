use axum::extract::{Path, State};
use uuid::Uuid;

use crate::handlers::error::{AppJson, Result};
use crate::models::todo::{Todo, TodoCreate, TodoUpdate};
use crate::AppState;

#[axum::debug_handler]
pub(crate) async fn todos_get(State(state): State<AppState>) -> Result<AppJson<Vec<Todo>>> {
    let state = state.read().unwrap();

    let todos: Vec<Todo> = state.todos.values().cloned().collect();
    Ok(AppJson(todos))
}

#[axum::debug_handler]
pub(crate) async fn todos_post(
    State(state): State<AppState>,
    AppJson(todo_create): AppJson<TodoCreate>,
) -> Result<AppJson<Todo>> {
    let mut state = state.write().unwrap();

    let id = Uuid::new_v4();
    let todo = Todo {
        id,
        text: todo_create.text,
        completed: false,
    };

    state.todos.insert(id, todo.clone());

    let created = state.todos.get(&id).unwrap();
    Ok(AppJson(created.clone()))
}

#[axum::debug_handler]
pub(crate) async fn todos_patch(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    AppJson(todo_update): AppJson<TodoUpdate>,
) -> Result<AppJson<Todo>> {
    let mut state = state.write().unwrap();

    let todo = state.todos.get_mut(&id).expect("Failed to find todo.");
    todo.completed = todo_update.completed;

    let updated = state.todos.get(&id).unwrap();
    Ok(AppJson(updated.clone()))
}

#[axum::debug_handler]
pub(crate) async fn todos_delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<AppJson<Todo>> {
    let mut state = state.write().unwrap();

    let deleted = state.todos.remove(&id).expect("Failed to find todo.");
    Ok(AppJson(deleted))
}
