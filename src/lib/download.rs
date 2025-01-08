use std::io;
use std::fs;

use chrono::{Local, DateTime};

pub async fn download_file(url: String, filename: String) {
    let response: reqwest::Response = reqwest::get(url).await.expect("Failed to download");
    // Check if the response status is success
    if response.status().is_success() {
        // Open a file in write mode
        let mut file = fs::File::create(filename.clone()).expect("Failed to create file");

        // Stream the response bytes to the file
        let content = response.bytes().await;
        match content {
            Ok(content) => {
                io::copy(&mut content.as_ref(), &mut file).expect("Failed to copy content");
                println!("Image downloaded and saved to {:?}", filename.clone());
            }
            Err(err) => {
                println!("Failed to parse image content {}", err);
            }
        }
    } else {
        println!("Failed to download image. Status: {:?}", response.status());
    }

    let file_metadata = get_file_metadata(filename.clone()).await;
    match file_metadata {
        Ok(metadata) => {
            if let Ok(time) = metadata.created() {
                let datetime: DateTime<Local> = time.into();
                println!("Created On: {:?}", datetime);
            } else {
                println!("Not supported on this platform or filesystem");
            }
        }
        Err(err) => {
            eprintln!("METADATA ERROR: {}", err);
        }
    }
}

async fn get_file_metadata(filename: String) -> Result<fs::Metadata, io::Error> {
    let file_metadata = fs::metadata(filename.clone())?;
    Ok(file_metadata)
}