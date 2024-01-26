build=`curl -s https://api.papermc.io/v2/projects/paper/versions/$1/builds | jq -r '.builds | last | .build'`
name=`curl -s https://api.papermc.io/v2/projects/paper/versions/$1/builds/$build | jq -r '.downloads.application.name'`
wget https://api.papermc.io/v2/projects/paper/versions/$1/builds/$build/downloads/$name -nv -O $2/Paper.jar