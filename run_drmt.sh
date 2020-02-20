#!/bin/bash

# TODO: When large_hw, drmt_latencies is not hardcoded,
# make them arguments
# $1 is DAG file, $2 is path to drmt dir
#file_name = `basename "$1"`
if [[ $# -ne 4 ]]
then
  echo "Usage: <DAG file> <path/to/drmt> <hw file> <dRMT latencies>"
else 
  echo "Arg 1: $1"
  echo "Arg 2: $2"

  echo "Arg 3: $3"

  echo "Arg 4: $4"
  cp $1 $2

  # Strip path 
  file_name=${1##*/}
  # Run drmt and strip file extension
#  echo $2/drmt.py
#  echo ${file_name%.py}
  python -u $2/drmt.py ${file_name%.py} $3 $4 10
fi

