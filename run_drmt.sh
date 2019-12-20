#!/bin/bash

# $1 is DAG file, $2 is path to drmt dir
#file_name = `basename "$1"`
if [[ $# -ne 2 ]]
then
  echo "Usage: <DAG file> <path/to/drmt>"
else 
  echo "Arg 1: $1"
  echo "Arg 2: $2"
  cp $1 $2
  cd $2

  # Strip path 
  file_name=${1##*/}
  # Run drmt and strip file extension
  python -u drmt.py ${file_name%.py} large_hw drmt_latencies 10
fi

