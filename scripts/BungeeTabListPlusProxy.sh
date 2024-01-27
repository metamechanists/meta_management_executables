cd $1
ls
wget https://api.spiget.org/v2/resources/313/download -nv -O tmp.zip
unzip -o tmp.zip
proxyname=`find . -type f -iname "BungeeTabListPlus-*"`
bukkitname=`find . -type f -iname "BungeeTabListPlus_Bukkit*"`
spongename=`find . -type f -iname "BungeeTabListPlus_Sponge*"`
rm tmp.zip
rm $bukkitname
rm $spongename
mv $proxyname BungeeTabListPlusProxy.jar