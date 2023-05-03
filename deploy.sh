#! /bin/bash

trunk build --release --public-url yew_app/

cp -r ./dist ./docs

touch ./docs/.nojekyll

git add .

git commit -m 'deploy'

git push