#!/bin/bash

prefix="btt-"
cmd=$1
extra=$2


if [[ "$cmd" == "edit-json" ]]; then
# TODO: find easier way 
  json=$(test -z "$extra" && echo "./out.json" || echo "$extra")
  cargo run --bin "${prefix}edit-tools" "$json"
elif [[ "$cmd" == "whiteout" ]]; then
  json=$(test -z "$extra" && echo "./out.json" || echo "$extra")
  cargo run --bin "${prefix}whiteout-labels" ./diagram.png "$json"
elif [[ "$cmd" == "gen-ocr" ]]; then
  cargo run --bin "${prefix}get-ocr" ./diagram.png > out.json
elif [[ "$cmd" == "show-labels" ]]; then
  cargo run --bin "${prefix}label-ocr" ./diagram.png ./out.json
elif [[ "$cmd" == "add-braille" ]]; then
  json=$(test -z "$extra" && echo "./out.json" || echo "$extra")
  cargo run --bin "${prefix}add-braille" ./out.png "$json" $3
fi
