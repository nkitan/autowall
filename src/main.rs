extern crate dotenv;
mod lib;
use lib::unsplash::integration as ui;
use lib::download::download_file;
use lib::place::place_wallpapers;

#[tokio::main]
async fn main() {
    dotenv::from_filename("autowall.env").ok();
    let wallpapers_dir: String = std::env::var("WALLPAPER_STORAGE_LOCATION").expect("WALLPAPER_STORAGE_LOCATION is not set!");
    let wallpapers_query: String = std::env::var("WALLPAPER_QUERY").expect("WALLPAPER_QUERY is not set!");

    // Query wallpapers_query and download 2 random photos to wallpapers_dir
    match ui::get_random_photo(2, wallpapers_query).await {
        Ok(photos) => {
            for photo in photos {
                let filename: String = format!("{}/{}.jpg", wallpapers_dir, photo.id);
                download_file(photo.urls.full, filename).await;
            }
        }
        Err(err) => eprintln!("ERROR: {}", err),
    }

    // Place wallpapers into windows wallpapers directory
    place_wallpapers();
}
