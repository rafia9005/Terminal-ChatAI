use crate::controller::send_chat_request;
use crate::models::Message;
use std::io::{self, Write};

mod controller;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

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

        match send_chat_request(messages.clone()).await {
            Ok(reply) => {
                println!("Megumin: {}", reply);

                messages.push(Message {
                    role: "assistant".to_string(),
                    content: reply,
                });
            }
            Err(err) => println!("{}", err),
        }
    }
    Ok(())
}
