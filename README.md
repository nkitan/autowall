# Autowall
Autowall is a simple Rust program to fetch wallpapers from Unsplash and provide them to Windows for use as slideshow backgrounds.
Windows allows users to set a slideshow as their wallpaper, picking a random picture from a specified folder.

## Features
- Fetches random wallpapers from Unsplash using a configurable query.
- Downloads wallpapers to a specified directory.
- Copies wallpapers to your Windows wallpapers folder (for WSL/dual-boot setups).
- Can be automated via cron.

## Usage
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
   DEST=/mnt/c/Users/<USERNAME>/Pictures/Wallpapers/
   SRC=/opt/autowall/wallpapers/
   ```
   Replace `<USERNAME>` with your Windows username.
4. **Set up environment variables**
   ```sh
   cp autowall.env.template autowall.env
   vim autowall.env
   ```
   Fill in your Unsplash API key and other settings.
5. **Use autowall**
   ```sh
   /opt/autowall/autowall
   ```

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

## Requirements
- Rust (see `Cargo.toml` for dependencies)
- Unsplash API key
- Linux environment (tested with WSL for Windows integration)

---
