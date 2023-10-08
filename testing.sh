#!/bin/bash


# Use a for loop to iterate through the files in the folder
for language in "Languages"/*; do
    for program in "$language"/*; do
        echo "Processing file: $program"
    done
done
