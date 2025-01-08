use std::process::Command;
use crate::lib::utils::get_current_datetime;

pub fn place_wallpapers() {
    // Create a new command that runs bash
    let output = Command::new("bash")
        .arg("scripts/place.sh")
        .output()
        .expect("Failed to place wallpapers");

    println!("[ {} ] Wallpapers Placed With Status {:?}", get_current_datetime(), output.status.code().unwrap());
}   