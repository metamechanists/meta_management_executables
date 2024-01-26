url=`curl -X GET https://blob.build/api/builds/$1/Dev/latest | jq -r .data.fileDownloadUrl`
wget $url -nv -O $2/$1.jar