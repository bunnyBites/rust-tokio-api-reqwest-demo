#[tokio::main]
async fn main() {
    let api_url = "https://cat-fact.herokuapp.com/facts/";

    // use await since get_mock_response returns a future trait
    let response = get_mock_response(api_url).await;

    match response {
        Ok(fetched_response) => {
            // prints the fetched response using debugging macro
            dbg!(fetched_response);
        }
        Err(_) => {
            // halts the program with a panic message
            panic!("Response Fetch Error");
        }
    }
}

async fn get_mock_response(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    let response = reqwest::get(url) // API GET request
        .await?
        .json::<serde_json::Value>() // deserialize response to Rust Data structure
        .await?;

    // Wrap response in Ok which is a variant of Result Enum
    Ok(response)
}
