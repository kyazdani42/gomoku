#!/bin/sh

npm i && npm run build
mv $(\find public -name '*.js') public/index.js
sed -ie 's/\"\/.*\.js\"/"\/index.js"/' ./public/index.html
rm -f ./public/index.htmle

cargo build --release
mv target/release/gomoku .

