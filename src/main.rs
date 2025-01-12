use std::{collections::HashMap, sync::OnceLock};

use axum::{
    extract::Query, routing::get, Json, Router
};
use ollama_rs::{
    generation::completion::request::GenerationRequest, Ollama
};

#[derive(Debug)]
struct Context {
    ollama: Ollama,
}

static CONTEXT: OnceLock<Context> = OnceLock::new();

impl Context {
    fn global() -> &'static Self {
        CONTEXT.get_or_init(|| {
            println!("Initializing Ollama...");
            Context {
                ollama: Ollama::default(),
            }
        })
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(chat));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn chat(Query(mut query): Query<HashMap<String, String>>) -> Json<String> {
    let prompt = if let Some(prompt) = query.remove("prompt") {
        prompt
    } else {
        return Json("Error: prompt not provided".to_string());
    };

    let req = GenerationRequest::new("qwen2.5:0.5b".into(), prompt);
    let res = Context::global().ollama.generate(req).await;
    Json(
        if let Ok(res) = res {
            res.response
        } else {
            "Error".to_string()
        }
    )
}
