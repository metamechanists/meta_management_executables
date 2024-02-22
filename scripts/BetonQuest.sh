url=`curl -s https://api.github.com/repos/BetonQuest/BetonQuest/releases/latest | jq -r ".assets[0].browser_download_url"`
wget $url -nv -O $1/BetonQuest.jar