#!/usr/bin/env bash

main () {
    printf "One for %s, one for me.\n" "${1:-you}"
    return 0
}

main "$@"
