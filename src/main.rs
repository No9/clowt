use std::io::Cursor;
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
    let questions_test_filename = "questions_test.csv";
    let questions_train_filename = "questions_train.csv";
    let questions_test_url = "https://raw.github.com/IBM/watson-machine-learning-samples/master/cloud/data/RAG/questions_test.csv";
    let questions_train_url = "https://raw.github.com/IBM/watson-machine-learning-samples/master/cloud/data/RAG/questions_train.csv";

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
}
