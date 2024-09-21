#!/bin/bash
# ./hide.sh <input-file-path> <file-to-hide-path>
mkdir -p out
ffmpeg -i $1 -metadata comment="$(base64 -w 0 $2)" out/meta.mp4
