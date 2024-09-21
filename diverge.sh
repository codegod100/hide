#!/bin/bash

# Check if two arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <file1> <file2>"
    exit 1
fi

file1="$1"
file2="$2"

# Check if files exist and are readable
if [ ! -f "$file1" ] || [ ! -r "$file1" ]; then
    echo "File $file1 does not exist or is not readable"
    exit 1
fi

if [ ! -f "$file2" ] || [ ! -r "$file2" ]; then
    echo "File $file2 does not exist or is not readable"
    exit 1
fi

# Read entire contents of files
content1=$(cat "$file1")
content2=$(cat "$file2")

# Check if contents are empty
if [ -z "$content1" ]; then
    echo "File $file1 is empty"
    exit 1
fi

if [ -z "$content2" ]; then
    echo "File $file2 is empty"
    exit 1
fi

# Convert contents to arrays of characters
IFS='' read -r -a arr1 <<< "$content1"
IFS='' read -r -a arr2 <<< "$content2"

# Find the length of the shorter content
len1=${#arr1[@]}
len2=${#arr2[@]}
min_len=$((len1 < len2 ? len1 : len2))

# Compare characters
for ((i=0; i<min_len; i++)); do
    if [[ "${arr1[i]}" != "${arr2[i]}" ]]; then
        echo "Contents diverge at position $i"
        echo "File1: '${arr1[i]}'"
        echo "File2: '${arr2[i]}'"
        exit 0
    fi
done

# If we've reached this point, one content is a prefix of the other
if [ $len1 -eq $len2 ]; then
    echo "Contents are identical"
elif [ $len1 -lt $len2 ]; then
    echo "Content of File1 is a prefix of File2. They diverge at position $len1"
else
    echo "Content of File2 is a prefix of File1. They diverge at position $len2"
fi
