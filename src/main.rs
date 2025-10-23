use std::{fs, path::{Path, PathBuf}};

use clap::Parser;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

use anki_exporter::Card;

const MODEL: &str = "schroneko/gemma-2-2b-jpn-it:latest";

pub async fn generate_card(client: &Ollama, tango: &str) -> Option<Card> {
    let meaning = client
        .generate(GenerationRequest::new(
            MODEL.to_string(),
            format!("Please translate「{}」to english. Provide nothing but the english translation.", tango),
        ))
        .await
        .unwrap();
    let example = client
        .generate(GenerationRequest::new(
            MODEL.to_string(),
            format!("「{}」の単語を使い、日本語で例文を一つ作ってください。例文以外のものを除きなさい。", tango),
        ))
        .await
        .unwrap();

    Card::new()
        .vocab(tango)
        .example(example.response.trim())
        .translation(meaning.response.trim())
        .construct()
}

#[derive(Parser)]
#[command(name = "Anki generator")]
#[command(about = "Generates Anki cards based on vocabulary.", long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            if let Some(file) = args.file {
                if !file.exists() {
                    panic!("File not found!")
                }
                let contents = tokio::fs::read_to_string(&file).await.unwrap();

                let mut set: tokio::task::JoinSet<Card> = tokio::task::JoinSet::new();

                for vocab in contents.lines() {
                    let vocab = vocab.to_owned();
                    set.spawn(async move {
                        let ollama = Ollama::default();
                        generate_card(&ollama, &vocab).await.unwrap()
                    });
                }

                let contents = set
                    .join_all()
                    .await
                    .iter()
                    .map(|card| {
                        let mut format = card.format_anki();
                        println!("FORMAT: [{}]", format);
                        format.push('\n');
                        format
                    })
                    .collect::<String>();
                println!("{}", contents);

                // file.parent().unwrap()

                tokio::fs::write("output.txt", &contents).await.unwrap();
            }
        });
}
