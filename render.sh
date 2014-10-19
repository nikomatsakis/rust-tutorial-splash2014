rm -rf html
mkdir html
mv index.md src/*html html
markdown < html/index.md > html/index.html

