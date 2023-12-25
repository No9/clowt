use polars::io::mmap::MmapBytesReader;
use polars::prelude::*;
use std::fs;
use std::io::Cursor;
use std::path::Path;

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
    let questions_test_filename = "./questions_test.csv";
    let questions_train_filename = "./questions_train.csv";
    let questions_test_url = "https://raw.github.com/IBM/watson-machine-learning-samples/master/cloud/data/RAG/questions_test.csv";
    let questions_train_url = "https://raw.github.com/IBM/watson-machine-learning-samples/master/cloud/data/RAG/questions_train.csv";

    if Path::new(questions_test_filename).exists() {
        fs::remove_file(questions_test_filename).expect("could not remove questions_test_filename");
    }
    if Path::new(questions_train_filename).exists() {
        fs::remove_file(questions_train_filename)
            .expect("could not remove questions_train_filename");
    }
    fetch_url(
        questions_test_url.to_string(),
        questions_test_filename.to_string(),
    )
    .await
    .unwrap();

    fetch_url(
        questions_train_url.to_string(),
        questions_train_filename.to_string(),
    )
    .await
    .unwrap();
    let file_test = std::fs::File::open(questions_test_filename).unwrap();
    let file_train = std::fs::File::open(questions_train_filename).unwrap();
    let file_test = Box::new(file_test) as Box<dyn MmapBytesReader>;
    let file_train = Box::new(file_train) as Box<dyn MmapBytesReader>;

    let _test_reader = CsvReader::new(file_test)
        .with_separator(b',')
        .has_header(true)
        .with_chunk_size(10)
        .batched_mmap(None)
        .unwrap();
    let _training_reader = CsvReader::new(file_train)
        .with_separator(b',')
        .has_header(true)
        .with_chunk_size(10)
        .batched_mmap(None)
        .unwrap();
}
