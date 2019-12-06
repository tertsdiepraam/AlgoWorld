cargo run wiki
git stash push dist/*
git checkout --orphan gh-pages
git stash pop
git add .
git commit -m "Deployed"
git push
git checkout master
