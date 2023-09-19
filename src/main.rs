#[tokio::main]
async fn main() {
    let api_url = "https://cat-fact.herokuapp.com/facts/";

    let response = get_mock_response(api_url).await;

    match response {
        Ok(fetched_response) => {
            dbg!(fetched_response);
        }
        Err(_) => {
            panic!("Response Fetch Error");
        }
    }
}

async fn get_mock_response(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;

    Ok(response)
}
