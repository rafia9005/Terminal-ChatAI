use reqwest::Client;
use serde::Serialize;
use std::env;
use std::io::{self, Write};
use tokio;

#[derive(Serialize, Clone)]
struct ChatRequest {
    messages: Vec<Message>,
    model: String,
    temperature: f32,
    max_tokens: u16,
    top_p: f32,
    stream: bool,
    stop: Option<String>,
}

#[derive(Serialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = env::var("GROQ_API_KEY").expect("API key not found in .env");
    let client = Client::new();
    let api_url = "https://api.groq.com/openai/v1/chat/completions";

    let mut messages: Vec<Message> = Vec::new();

    println!("AI Chatbot. Ketik 'exit' untuk keluar.");
    loop {
        print!("You: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        messages.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        let request_body = ChatRequest {
            messages: messages.clone(),
            model: "llama3-8b-8192".to_string(),
            temperature: 1.0,
            max_tokens: 1024,
            top_p: 1.0,
            stream: false,
            stop: None,
        };

        let response = client
            .post(api_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request_body)
            .send()
            .await?;

        let body = response.text().await?;

        if body.is_empty() {
            println!("AI: [Error: Empty response from API]");
        } else {
            let response_text: serde_json::Value = serde_json::from_str(&body)?;

            if let Some(reply) = response_text["choices"][0]["message"]["content"].as_str() {
                println!("AI: {}", reply);

                messages.push(Message {
                    role: "assistant".to_string(),
                    content: reply.to_string(),
                });
            } else {
                println!("AI: [Error: No valid response content]");
            }
        }
    }

    Ok(())
}
