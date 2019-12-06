cargo run wiki
git stash push dist/*
git checkout -b gh-pages
git stash pop
git add .
git commit -m "Deployed"
git push
git checkout master
