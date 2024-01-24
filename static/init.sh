#!/bin/bash
# This script copies static files to the dist folder to node_modules

HTMX_VERSION=$(jq -r '.dependencies."htmx.org"' package.json)
HYPERSCRIPT_VERSION=$(jq -r '.dependencies."hyperscript.org"' package.json)
IDIOMORPH_VERSION=$(jq -r '.dependencies."idiomorph"' package.json)

# Check if node_modules folder exists
if [ ! -d "node_modules" ]; then
  echo "node_modules folder does not exist. Please run npm install first."
  exit 1
fi

# Clear folder if dist folder exists
if [ -d "dist" ]; then
  rm -rf dist
fi
mkdir dist

cp -r ./node_modules/htmx.org/dist/htmx.min.js "./dist/htmx-$HTMX_VERSION.min.js"
cp -r ./node_modules/htmx.org/dist/ext/response-targets.js "./dist/htmx-response-targets-$HTMX_VERSION.min.js"
cp -r ./node_modules/hyperscript.org/dist/_hyperscript.min.js "./dist/_hyperscript-$HYPERSCRIPT_VERSION.min.js"
cp -r ./node_modules/idiomorph/dist/idiomorph-ext.min.js "./dist/idiomorph-$IDIOMORPH_VERSION.min.js"
