use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ChatRequest {
    pub messages: Vec<Message>,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u16,
    pub top_p: f32,
    pub stream: bool,
    pub stop: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}
