use polars::prelude::*;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};
use std::io::Cursor;
use std::path::Path;
use tokio::task;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn fetch_url(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Get Files test and training data
    let questions_test_filename = "./questions_test.csv";
    let questions_train_filename = "./questions_train.csv";
    let questions_test_url = "https://raw.github.com/IBM/watson-machine-learning-samples/master/cloud/data/RAG/questions_test.csv";
    let questions_train_url = "https://raw.github.com/IBM/watson-machine-learning-samples/master/cloud/data/RAG/questions_train.csv";

    if !Path::new(questions_test_filename).exists() {
        fetch_url(
            questions_test_url.to_string(),
            questions_test_filename.to_string(),
        )
        .await
        .unwrap();
    }
    if !Path::new(questions_train_filename).exists() {
        fetch_url(
            questions_train_url.to_string(),
            questions_train_filename.to_string(),
        )
        .await
        .unwrap();
    }

    // Load files into MmapBytesReaders
    // let file_test = std::fs::File::open(questions_test_filename).unwrap();
    // let file_train = std::fs::File::open(questions_train_filename).unwrap();
    // let file_test = Box::new(file_test) as Box<dyn MmapBytesReader>;
    // let file_train = Box::new(file_train) as Box<dyn MmapBytesReader>;

    let test_reader = LazyCsvReader::new(questions_test_filename)
        .with_separator(b',')
        .has_header(true)
        .finish()
        .unwrap();
    let _training_reader = LazyCsvReader::new(questions_train_filename)
        .with_separator(b',')
        .has_header(true)
        .finish()
        .unwrap();
    let out = test_reader
        .clone()
        .select([col("*")])
        .limit(5)
        .collect()
        .unwrap();
    // Query training reader to print data
    println!("{}", out);

    // Load Knowledge base
    let documents_filename = "./psgs.tsv";
    let documents_url =
        "https://raw.github.com/IBM/watson-machine-learning-samples/master/cloud/data/RAG/psgs.tsv";
    if !Path::new(documents_filename).exists() {
        fetch_url(documents_url.to_string(), documents_filename.to_string())
            .await
            .unwrap();
    }

    // the document does contain a header so should it be included in the rows?

    let documents = LazyCsvReader::new(documents_filename)
        .with_separator(b'\t')
        .has_header(true)
        .finish()
        .unwrap();
    let out = documents
        .clone()
        .select([col("*")])
        .limit(5)
        .collect()
        .unwrap();
    println!("{}", out);

    let _res = task::spawn_blocking(move || {
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
            .create_model()
            .unwrap();

        // Define input
        let sentences = ["this is an example sentence", "each sentence is converted"];

        // Generate Embeddings
        let embeddings = model.encode(&sentences).unwrap();
        println!("{embeddings:?}");
        // Stand-in for compute-heavy work or using synchronous APIs
        // v.push_str("world");
        // // Pass ownership of the value back to the asynchronous context
        // v
    })
    .await
    .unwrap();

    // Create an embedding function
    // https://github.com/guillaume-be/rust-bert/blob/1f4d344668232da8e669e7fea1391c8829d5d1e3/examples/sentence_embeddings.rs

    // Define Model on watsonx ???

    // Connect to elastic instance

    // Create elasticsearch store in connector

    // Embed and index documents

    // Generate a retrieval-augmented response to a question
}
