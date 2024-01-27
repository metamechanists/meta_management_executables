url=`curl -X GET https://blob.build/api/builds/DynaTech/Main/latest | jq -r .data.fileDownloadUrl`
wget $url -nv -O $2/DynaTech.jar