#!/bin/bash

if [ $# -eq 0 ]; then
    echo "No arguments supplied"
    exit 1
fi

if [[ $(git diff --staged | wc -c) -ne 0 ]]; then
    echo "Git has staged files! Please unstage them first."
    exit 1
fi

SUBCRATE_NAME=$1

echo "Creating template directory..."
cp -rv crate-template retina-$SUBCRATE_NAME
cd retina-$SUBCRATE_NAME

echo "Templating retina-$SUBCRATE_NAME..."
find . -type f | xargs sed -i  "s/SUBCRATE/$SUBCRATE_NAME/g"

echo "Commiting to git..."
git add .
git commit -m "[$SUBCRATE_NAME] Reserving crates.io crate"

echo "Publishing to crates.io..."
cargo publish
