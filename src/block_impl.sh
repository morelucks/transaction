#!/usr/bin/ sh

BINARY_FILE=""

echo "downloading binary file"

wget  "$BINARY_FILE"

echo "download completed."

chmod +x blocks_impl

sudo mv blocks_impl "/usr/local/bin/luckify"