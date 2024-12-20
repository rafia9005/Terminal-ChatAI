use crate::models::{ChatRequest, Message};
use reqwest::Client;
use serde_json::Value;
use std::env;

pub async fn send_chat_request(
    messages: Vec<Message>,
) -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = env::var("GROQ_API_KEY").expect("API key not found in .env");
    let client = Client::new();
    let api_url = env::var("API_URL").expect("API URL not found in .env");

    let system_message = Message {
        role: "system".to_string(),
        content: "Kamu berbahasa Indonesia dan selalu menjawab dalam bahasa Indonesia. Kamu diciptakan oleh Ahmad Rafi, yang juga dikenal dengan nama pengguna rafia9005 di GitHub: https://github.com/rafia9005. Jika ada yang bertanya siapa yang menciptakan kamu, jawab dengan menyebutkan nama saya, Ahmad Rafi, dan sertakan link GitHub saya. Nama kamu adalah Megumin. Jawaban kamu harus singkat, kecuali jika diminta untuk menjelaskan dengan detail.".to_string().to_uppercase(),
    };

    let mut all_messages = vec![system_message];
    all_messages.extend(messages);

    let request_body = ChatRequest {
        messages: all_messages,
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
        return Err("AI: [Error: Empty response from API]".into());
    }

    let response_text: Value = serde_json::from_str(&body)?;

    if let Some(reply) = response_text["choices"][0]["message"]["content"].as_str() {
        let clean_reply = reply.replace("```", "").trim().to_string();
        Ok(clean_reply)
    } else {
        Err("AI: [Error: No valid response content]".into())
    }
}

