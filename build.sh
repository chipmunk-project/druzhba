#!/bin/bash

# Note: This build script is only a temporary
# solution. The original solution using
# build.rs was not suitable because of file locking.

# The first five inputs are for dgen and the 
# next two are for druzhba. Dgen takes in 
# spec name, ALU files, pipeline depth/width.
# Druzhba takes in number of ticks and hole
# configuration file from Chipmunk.

if [ "$#" -ne 10 ]
then
  echo "Wrong number of inputs. $# supplied."
else
  cd dgen
  # Dgen inputs:
  #   Spec name (eg. simple, blue_increase, etc.)
  #   Stateful ALU 
  #   Stateless ALU
  #   Pipeline width
  #   Pipeline depth
  #   Stateful ALUs per stage
  cargo build && cargo run $1 ../$2 ../$3 $4 $5 $6 $7
  cd ..
  # Druhzba inputs:
  #   Hole configuration file (i.e. any file in hole_configurations)
  #   Containers per Phv
  #   Ticks
  cargo build && cargo run $8 $9 $(10)

fi
