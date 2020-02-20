#!/bin/bash
echo "Arg 1: $1"
p4-graphs --split-match-action-events $1
