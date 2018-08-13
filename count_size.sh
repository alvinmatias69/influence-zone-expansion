#!/bin/bash
FILES=./output/*
touch size.csv
echo "interest points,size" > size.csv
for f in $FILES
do
  FILESIZE=$(stat -f%z "$f")
  FILENAME=$(basename -- "$f")
  FILENAME="${FILENAME%.*}"
  echo "$FILENAME,$FILESIZE" >> size.csv
done
