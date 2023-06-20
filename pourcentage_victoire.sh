#!/bin/bash

BLUEWIN=0
REDWIN=0

if [[ -z "$1" ]]
then
    echo "Correct usage: pourcentage_victoire_script.sh <number>"
    exit 1
fi

for i in $(seq 1 $1)
do
    cargo run --bin blobwar 2> /dev/null | grep -e "wins over BLUE"
    if [ $? -eq 0 ]
    then
        REDWIN=$((REDWIN + 1))
        echo RED
    else
        BLUEWIN=$((BLUEWIN + 1))
        echo BLUE
    fi
done

echo "Red: $REDWIN / Blue: $BLUEWIN"
echo `bc -l <<< $REDWIN' / $1'`