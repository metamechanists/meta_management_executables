url=`curl -s https://api.github.com/repos/KaspianDev/AntiPopup/releases/latest | jq -r ".assets[0].browser_download_url"`
wget $url -nv -O $1/AntiPopup.jar