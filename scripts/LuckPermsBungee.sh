url=`curl -s https://metadata.luckperms.net/data/all | jq .downloads.bungee`
wget $url -nv -0 $1/LuckPermsBungee.jar