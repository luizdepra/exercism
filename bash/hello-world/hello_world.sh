#!/usr/bin/env bash

set -o errexit
set -o nounset

main() {
  printf "Hello, World!"
  return 0
}

main "$@"
