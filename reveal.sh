#!/bin/bash
# ./reveal.sh <input-file-path> <file-to-reveal-path>
mkdir -p out
ffprobe -show_entries format_tags=comment -i "$1" -of compact=p=1:nk=1 > out/meta.txt
sed 's/format|//' out/meta.txt > out/b64.txt
base64 -di out/b64.txt > "$2"
