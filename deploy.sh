cargo run wiki
git stash push dist/*
git checkout --orphan gh-pages
git checkout gh-pages
git stash pop
git mv dist/* .
git add -A
git commit -m "Deployed"
git push
git checkout master
