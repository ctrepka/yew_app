#! /bin/bash

trunk build --release --public-url yew_app/

rm -rf docs

cp -a dist/. docs

touch ./docs/.nojekyll

git add .

git commit -m 'deploy'

git push