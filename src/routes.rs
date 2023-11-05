use crate::{models::*, AppState};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    Form,
};
use minijinja::{context, Environment};
use rand::{distributions::Alphanumeric, Rng};
use serde::ser::Serialize;
use std::sync::Arc;

pub fn render_html<S: Serialize>(template_name: &str, context: S, jinja_env: &Environment) -> Option<Html<String>> {
    // TODO Replace unwraps with better error handling
    let tpl = jinja_env.get_template(template_name).unwrap();
    let content = tpl.render(context).unwrap();
    return Some(Html(content));
}

pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let todos: Vec<TodoItem> = sqlx::query_as!(TodoItem, "SELECT * FROM todos")
        .fetch_all(&state.db)
        .await
        .unwrap();
    return render_html("home.html", context!(todos), &state.jinja).unwrap();
}

pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Form(form): Form<TodoItem>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let title_clone = form.title.clone();

    let query_result = sqlx::query!("INSERT INTO todos (title,date) VALUES (?, ?)", form.title, form.date)
        .execute(&state.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        println!("Could not execute insert due to error: {}", err);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Unknown error")));
    }

    return Ok(format!("Todo item '{}' succesfuly added", title_clone));
}

pub async fn login(State(state): State<Arc<AppState>>) -> Html<String> {
    return render_html("login.html", context!(cur_url => "login"), &state.jinja).unwrap();
}

pub async fn json() -> Json<Info> {
    return Json(Info {
        name: String::from("Mike"),
        age: 24,
    });
}

pub async fn json_list() -> Json<Vec<Info>> {
    let mut vec = Vec::new();

    for i in 0..5_000u32 {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();
        vec.push(Info { name: s, age: i });
    }
    return Json(vec);
}
