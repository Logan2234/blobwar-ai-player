#!/bin/bash

TEMPS=0

if [[ -z "$1" ]]
then
    echo "Correct usage: temps_exec_script.sh <number>"
    exit 1
fi

# cargo run --quiet --bin blobwar > /dev/null 2>/dev/null

for i in $(seq 1 $1)
do
    echo $i
    MYVARIABLE=`\time -f "%e" cargo run --quiet --bin blobwar 2>&1 > /dev/null`
    TEMPS=`bc -l <<< $TEMPS" + "$MYVARIABLE`
done

echo `bc -l <<< $TEMPS" / "$1`