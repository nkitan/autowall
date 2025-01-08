# Autowall
Autowall is a simple program to fetch wallpapers from unsplash and provide them to windows to use as backgrounds for the slideshow.  
Windows allows users to set a slideshow as their wallpaper. This picks up a random picture from the specified folder and shows it as the desktop wallpaper.  
  
### Usage
1. Clone the repo  
   ```git clone git@github.com:nkitan/autowall.git```
2. Run setup  
   ```bash setup.sh```
   Provide sudo permissions if required
3. Set up place.sh  
    ```vim place.sh```
     
    Set DEST as the windows wallpapers folder  
    Set SRC as the wallpapers directory inside the cloned autowall folder
5. Set up environment variables  
    ```cp autowall.env.template autowall.env```  
    ```vim autowall.env```
6. Use autowall  
   ```/opt/autowall/autowall```  

### Automate
To make this run every day, place the scripts/autowall-cron file inside /etc/cron.daily
or
set up autowall to run every day using cron manually
