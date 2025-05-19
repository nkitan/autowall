use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub width: u32,
    pub height: u32,
    pub color: String,
    pub blur_hash: String,
    pub downloads: u32,
    pub likes: u32,
    pub liked_by_user: bool,
    pub description: Option<String>,
    pub exif: Exif,
    pub location: Option<Location>,
    pub current_user_collections: Vec<Collection>,
    pub urls: Urls,
    pub links: PhotoLinks,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exif {
    pub make: Option<String>,
    pub model: Option<String>,
    pub exposure_time: Option<String>,
    pub aperture: Option<String>,
    pub focal_length: Option<String>,
    pub iso: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub position: Option<Position>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub id: u32,
    pub title: String,
    pub published_at: String,
    pub last_collected_at: String,
    pub updated_at: String,
    pub cover_photo: Option<String>, // Adjust type if more information is available
    pub user: Option<String>,        // Adjust type if more information is available
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Urls {
    pub raw: String,
    pub full: String,
    pub regular: String,
    pub small: String,
    pub thumb: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoLinks {
    #[serde(rename = "self")]
    pub self_link: String, // Renamed `self` to `self_link` as `self` is a reserved keyword
    pub html: String,
    pub download: String,
    pub download_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub updated_at: String,
    pub username: String,
    pub name: String,
    pub portfolio_url: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub total_likes: u32,
    pub total_photos: u32,
    pub total_collections: u32,
    pub instagram_username: Option<String>,
    pub twitter_username: Option<String>,
    pub links: UserLinks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLinks {
    #[serde(rename = "self")]
    pub self_link: String, // Renamed `self` to `self_link` as `self` is a reserved keyword
    pub html: String,
    pub photos: String,
    pub likes: String,
    pub portfolio: String,
}