url=`curl -s https://api.github.com/repos/nulli0n/NexEngine-spigot/releases/latest | jq -r ".assets[0].browser_download_url"`
wget $url -nv -O $1/NexEngine.jar