# Autowall
Autowall is a simple Rust program to fetch wallpapers from Unsplash and provide them to Windows for use as slideshow backgrounds.  
Windows allows users to set a slideshow as their wallpaper, picking a random picture from a specified folder.  
Autowall fetches and stores wallpapers from unsplash and stores it in a folder which is picked up by windows.

## Features
- Fetches random wallpapers from Unsplash using a configurable query.
- Downloads wallpapers to a specified directory.
- Copies wallpapers to your Windows wallpapers folder (for WSL/dual-boot setups).
- Can be automated via cron.

## Requirements
- Rust (see `Cargo.toml` for dependencies)
- Unsplash API key
- Any Linux System (for dual-boot systems) / WSL Distro (Windows Subsystem for Linux)

## Usage - for Dual boot / WSL setups
1. **Clone the repo**
   ```sh
   git clone git@github.com:nkitan/autowall.git
   cd autowall
   ```
2. **Run setup**
   ```sh
   bash setup.sh
   ```
   Provide sudo permissions if required.
3. **Configure `scripts/place.sh`**
   Edit the destination and source directories in `scripts/place.sh`:
   ```sh
   DEST=/mnt/c/Users/<USERNAME>/Pictures/Wallpapers/ for WSL 
   or 
   DEST=/mnt/windows_drive/Users/<USERNAME>/Pictures/Wallpapers/ for dualboot (Please note that for dual boot systems it is assumed that windows install drive is mounted at /mnt/windows_drive)
   SRC=/opt/autowall/wallpapers/
   ```
   Replace `<USERNAME>` with your Windows username.
4. **Set up environment variables**
   ```sh
   cp autowall.env.template autowall.env
   vim autowall.env
   ```
   Fill in your Unsplash API key and other settings.
5. **Use autowall** to fetch a new wallpaper
   ```sh
   /opt/autowall/autowall
   ```
6. Set Windows to pick up wallpapers from C:/Users/<USERNAME>/Pictures/Wallpapers/

## Automation
To run Autowall automatically (e.g., daily):

- Copy `scripts/autowall-cron` to `/etc/cron.daily/`:
  ```sh
  sudo cp scripts/autowall-cron /etc/cron.daily/autowall
  sudo chmod +x /etc/cron.daily/autowall
  ```
  This script will run Autowall and append logs to `/opt/autowall/autowall.log`.

Or set up your own cron job to run `/opt/autowall/autowall` as needed.

## File Structure
- `src/main.rs`: Main entry point.
- `src/lib/unsplash/`: Unsplash API integration and schema.
- `src/lib/download.rs`: Download logic.
- `src/lib/place.rs`: Wallpaper placement logic.
- `scripts/place.sh`: Copies wallpapers to Windows folder.
- `scripts/autowall-cron`: Example cron script.
- `autowall.env.template`: Example environment config.

Please note that wallpapers will be updated when linux is running, if linux was not run even once, say in case of dual boot, then autowall would not have been run and as a result no new wallpapers will be added
---
