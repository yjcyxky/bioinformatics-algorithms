#!/bin/bash

input=$1
output=$2

if [ -z "$output" ]; then
  output=$input.output
fi

if [ ! -f "$input" ]; then
  echo "ERROR: $input doesn't exist."
  exit
fi

if [ -f "$output" ]; then
  echo "ERROR: $output exists."
  exit
fi

while read line
do
  new_line="vec![$(echo $line | sed 's/ -> /,/g')],"
  echo $new_line >> $output
done < $input