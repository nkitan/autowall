use reqwest::header::USER_AGENT;
use reqwest::Error;
use crate::lib;
use lib::unsplash::schema::Photo;

pub async fn get_random_photo(count: u8, query: String) -> Result<Vec<Photo>, Error> { 
    let unsplash_access_key = std::env::var("UNSPLASH_ACCESS_KEY").expect("UNSPLASH_ACCESS_KEY must be set.");
    let unsplash_base_url = std::env::var("UNSPLASH_BASE_URL").expect("UNSPLASH_BASE_URL must be set.");
    let user_agent = std::env::var("USER_AGENT").expect("USER_AGENT must be set.");
    let request_url = format!("https://{base_url}/photos/random?count={count}&orientation={orientation}&query={query}",
                              base_url = unsplash_base_url,
                              count = count,
                              orientation = "landscape",
                              query = query,
    );
    let client = reqwest::Client::new();
    let response = client
        .get(request_url)
        .header("Accept-Version", "v1")
        .header(USER_AGENT, user_agent)
        .header("Authorization", format!("Client-ID {}", unsplash_access_key))
        .send()
        .await?;
    
    let response_text = response.text().await?;
    let photos: Vec<Photo> = serde_json::from_str(response_text.as_str()).expect("Failed to parse JSON");
    Ok(photos)
}

