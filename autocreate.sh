#!/bin/bash

# create a new cargo binary project from a template for a puzzle.


RED="\e[31m"
ORANGE="\e[33m"
GREEN="\e[32m"
CYAN="\e[96m"
CRESET="\e[0m"

TEMPLATE="template.rs"

usage() {
    echo "Usage: $1 <daynumber> <puzzle>"
    echo "example of daynumber : 2, 02, 12"
    echo "puzzle : A or B"
    exit 1
}

if [ $# -ne 2 ]; then
    usage "$0"
fi

day=$(printf "%02d" $(( 10#$1)) )
day_no_zero=$(printf "%d" $(( 10#$1)) )
puzzle="$2"
pb="$day-$puzzle" # problem

cargo new "day_$pb" || {
    echo -e "$RED Could not create new project $CRESET"
    exit 1
}

cp "$TEMPLATE" "day_$pb/src/main.rs" && sed -i "s/<DAY>/${day_no_zero}/" "day_$pb/src/main.rs"




