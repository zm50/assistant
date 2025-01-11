use ollama_rs::{
    generation::completion::request::GenerationRequest, Ollama
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();

    let req = GenerationRequest::new("qwen2.5:0.5b".into(), "Hello, world!".to_string());

    let res = ollama.generate(req).await?;
    println!("Response: {}", res.response);
    Ok(())
}
