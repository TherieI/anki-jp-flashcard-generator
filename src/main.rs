use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

use anki_exporter::Card;

const MODEL: &str = "schroneko/gemma-2-2b-jpn-it:latest";

const TANGO: &str = "攻撃";

pub async fn generate_card(client: &Ollama, tango: &str) -> Option<Card> {
    let meaning = client
        .generate(GenerationRequest::new(
            MODEL.to_string(),
            format!("Do not use any excess formatting in your response.\nPlease translate「{}」to english.", tango),
        ))
        .await
        .unwrap();
    let example = client
        .generate(GenerationRequest::new(
            MODEL.to_string(),
            format!("過剰な書式設定を使わないでください。「{}」の単語を使い、日本語で例文を一つ作ってください。", tango),
        ))
        .await
        .unwrap();

    Card::new()
        .vocab(tango)
        .example(example.response.trim())
        .translation(meaning.response.trim())
        .construct()
}

fn main() {
    let ollama = Ollama::default();

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let card = generate_card(&ollama, TANGO).await.unwrap();
            println!("{}", card.format_anki())
        });
}
