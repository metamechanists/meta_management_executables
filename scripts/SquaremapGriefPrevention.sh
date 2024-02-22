`wget -nv https://nightly.link/jpenilla/squaremap-addons/workflows/build/master/artifacts.zip`
unzip artifacts.zip
mv artifacts/squaremap-griefprevention-1.0.0-SNAPSHOT.jar $1/SquaremapGriefprevention.jar
rm artifacts -R
rm artifacts.zip