#!/usr/bin/bash

# Select which week you want to test against (e.g. './test_cases.sh week1')
# All valid cases should exit with code 0
# All invalid cases should exit with code 1

# Point to dir with test weeks
caseFolder="cases"

# Keep track of number of failed tests
failedTests=0

# Test if folder exists
if [ -d "${caseFolder}/$1" ]; then

    echo "Testing valid cases..."
    validFolder="${caseFolder}/$1/valid/*.c"
    for file in $validFolder; do
        echo "Testing $file..."
        cargo run --quiet -- $file
        if [[ "$?" == "0" ]]; then
            echo "Success!"
        else
            echo "Failure!"
            let "failedTests += 1"
        fi
    done

    echo ""

    echo "Testing invalid cases..."
    invalidFolder="${caseFolder}/$1/invalid/*.c"
        for file in $invalidFolder; do
        echo "Testing $file..."
        cargo run --quiet -- $file
        if [[ "$?" == "1" ]]; then
            echo "Failed successfully!"
        else
            echo "Failure!"
            let "failedTests += 1"
        fi
    done
    
    echo ""

    # Print result
    if [[ "$failedTests" == "0" ]]; then
        echo "All tests succeeded!"
    else
        echo "$failedTests tests failed."
    fi

# Folder doesn't even exist
else

    echo "Specified folder does not exist."

fi
