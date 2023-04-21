#!/usr/bin/env bash

main () {
    if (( $# != 2 )); then
        echo "Usage: hamming.sh <string1> <string2>"
        return 1
    fi

    local diff=0
    local left=$1
    local right=$2

    if (( ${#left} != ${#right} )); then
        echo "left and right strands must be of equal length"
        return 1
    fi

    for (( i=0; i<${#left}; i++ )); do
        if [[ "${left:$i:1}" != "${right:$i:1}" ]]; then
            (( diff++ ))
        fi
    done

    echo "${diff}"
}

main "$@"
