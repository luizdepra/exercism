#!/usr/bin/env bash

main () {
    if (( $# != 1 )); then
        echo "Usage: error_handling.sh <person>"
        return 1
    fi

    echo "Hello, $1"
}

main "$@"
